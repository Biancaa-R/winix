use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};
use std::io::{self, BufRead};
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};

// Sync version for benchmarking
pub fn head_sync<S: AsRef<Path>>(files: Vec<S>, lines: usize) -> io::Result<String> {
    let mut result = String::new();
    let mut total_lines = 0;

    for file_path in files {
        if total_lines >= lines {
            break;
        }

        let file = std::fs::File::open(&file_path)?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            if total_lines >= lines {
                break;
            }
            let mut line = line?;
            // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
            if line.ends_with('\r') {
                line.pop(); // Remove '\r'
            }
            result.push_str(&line);
            result.push('\n');
            total_lines += 1;
        }
    }

    Ok(result)
}

// Async version that returns a Stream<Bytes>
pub async fn head_async<S: AsRef<Path> + Send + 'static>(
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

                stream::unfold(
                    (lines_stream, 0, lines),
                    |(mut lines, count, max_lines)| async move {
                        if count >= max_lines {
                            return None;
                        }

                        match lines.next_line().await {
                            Ok(Some(line)) => {
                                let mut normalized_line = line;
                                // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
                                if normalized_line.ends_with('\r') {
                                    normalized_line.pop(); // Remove '\r'
                                }
                                normalized_line.push('\n');
                                Some((
                                    Ok(Bytes::from(normalized_line)),
                                    (lines, count + 1, max_lines),
                                ))
                            }
                            Ok(None) => None,
                            Err(e) => Some((Err(e), (lines, count, max_lines))),
                        }
                    },
                )
                .boxed()
            }
            Err(e) => stream::once(async move { Err(e) }).boxed(),
        }
    }
    .await
}

// Convenience function that collects the stream into a String
pub async fn head_async_to_string<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
    lines: usize,
) -> io::Result<String> {
    let mut result = String::new();
    let mut stream = head_async(files, lines).await;

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
    fn test_head_sync() {
        let file_path = "test_head.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        std::fs::write(file_path, content).unwrap();

        let result = head_sync(vec![file_path], 3).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");

        std::fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    async fn test_head_async() {
        let file_path = "test_head_async.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        tokio::fs::write(file_path, content).await.unwrap();

        let result = head_async_to_string(vec![file_path], 3).await.unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");

        tokio::fs::remove_file(file_path).await.unwrap();
    }
}
