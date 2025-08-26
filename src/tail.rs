use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};
use std::io::{self, BufRead};
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};

// Sync version for benchmarking
pub fn tail_sync<S: AsRef<Path>>(files: Vec<S>, lines: usize) -> io::Result<String> {
    let mut result = String::new();

    for file_path in files {
        let file = std::fs::File::open(&file_path)?;
        let reader = std::io::BufReader::new(file);
        let all_lines: Vec<String> = reader
            .lines()
            .map(|line| {
                let mut line = line?;
                // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
                if line.ends_with('\r') {
                    line.pop(); // Remove '\r'
                }
                Ok(line)
            })
            .collect::<io::Result<Vec<String>>>()?;

        let start = if all_lines.len() > lines {
            all_lines.len() - lines
        } else {
            0
        };

        for line in &all_lines[start..] {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

// Async version that returns a Stream<Bytes>
pub async fn tail_async<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
    lines: usize,
) -> impl Stream<Item = io::Result<Bytes>> {
    if files.is_empty() {
        return stream::empty().boxed();
    }

    let path = files[0].as_ref().to_path_buf();

    async move {
        match TokioFile::open(&path).await {
            Ok(file) => {
                let reader = TokioBufReader::new(file);
                let lines_stream = reader.lines();

                // First, collect all lines to determine which ones to output
                let mut all_lines = Vec::new();
                let mut lines_iter = lines_stream;

                while let Ok(Some(line)) = lines_iter.next_line().await {
                    let mut normalized_line = line;
                    // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
                    if normalized_line.ends_with('\r') {
                        normalized_line.pop(); // Remove '\r'
                    }
                    all_lines.push(normalized_line);
                }

                let start = if all_lines.len() > lines {
                    all_lines.len() - lines
                } else {
                    0
                };

                // Create a stream from the last N lines
                let tail_lines = all_lines[start..].to_vec();
                stream::iter(tail_lines.into_iter().map(|line| {
                    let output = format!("{}\n", line);
                    Ok(Bytes::from(output))
                }))
                .boxed()
            }
            Err(e) => stream::once(async move { Err(e) }).boxed(),
        }
    }
    .await
}

// Convenience function that collects the stream into a String
pub async fn tail_async_to_string<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
    lines: usize,
) -> io::Result<String> {
    let mut result = String::new();
    let mut stream = tail_async(files, lines).await;

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                if let Ok(chunk_str) = String::from_utf8(bytes.to_vec()) {
                    result.push_str(&chunk_str);
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_sync() {
        let file_path = "test_tail.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        std::fs::write(file_path, content).unwrap();

        let result = tail_sync(vec![file_path], 3).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 3");
        assert_eq!(lines[1], "line 4");
        assert_eq!(lines[2], "line 5");

        std::fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    async fn test_tail_async() {
        let file_path = "test_tail_async.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        tokio::fs::write(file_path, content).await.unwrap();

        let result = tail_async_to_string(vec![file_path], 3).await.unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 3");
        assert_eq!(lines[1], "line 4");
        assert_eq!(lines[2], "line 5");

        tokio::fs::remove_file(file_path).await.unwrap();
    }
}
