use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};

// === Sync implementation ===
#[allow(dead_code)]
pub fn cat<S: AsRef<Path>>(files: Vec<S>) -> io::Result<String> {
    let mut result = String::new();

    for file_path in files {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line?;
            if line.ends_with('\r') {
                line.pop();
            }
            result.push_str(&line);
            result.push('\n');
        }
    }

    Ok(result)
}

#[allow(dead_code)]
// === Async stream version ===
pub async fn cat_async<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
) -> impl Stream<Item = io::Result<Bytes>> {
    if files.is_empty() {
        return stream::empty().boxed();
    }

    let mut all_streams = Vec::new();

    for file_path in files {
        let path = file_path.as_ref().to_path_buf();
        let file_stream = async move {
            match TokioFile::open(&path).await {
                Ok(file) => {
                    let reader = TokioBufReader::new(file);
                    let lines = reader.lines();

                    Ok::<_, io::Error>(stream::unfold(lines, |mut lines| async move {
                        match lines.next_line().await {
                            Ok(Some(line)) => {
                                let mut normalized = line;
                                if normalized.ends_with('\r') {
                                    normalized.pop();
                                }
                                normalized.push('\n');
                                Some((Ok(Bytes::from(normalized)), lines))
                            }
                            Ok(None) => None,
                            Err(e) => Some((Err(e), lines)),
                        }
                    }))
                }
                Err(e) => Err(e),
            }
        };
        all_streams.push(file_stream);
    }

    if let Some(first_stream) = all_streams.into_iter().next() {
        match first_stream.await {
            Ok(stream) => stream.boxed(),
            Err(e) => stream::once(async move { Err(e) }).boxed(),
        }
    } else {
        stream::empty().boxed()
    }
}

#[allow(dead_code)]
// === Async version returning String ===
pub async fn cat_async_to_string<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
) -> io::Result<String> {
    let mut result = String::new();

    for file_path in files {
        let path = file_path.as_ref().to_path_buf();

        match TokioFile::open(&path).await {
            Ok(file) => {
                let reader = TokioBufReader::new(file);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let mut normalized = line;
                    if normalized.ends_with('\r') {
                        normalized.pop();
                    }
                    normalized.push('\n');
                    result.push_str(&normalized);
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

// === Benchmarking ===
#[allow(dead_code)]
pub async fn benchmark_cat_sync_vs_async<S: AsRef<Path> + Send + Clone + 'static>(
    files: Vec<S>,
) -> (std::time::Duration, std::time::Duration) {
    use std::time::Instant;

    let start = Instant::now();
    let _ = cat(files.clone());
    let sync_dur = start.elapsed();

    let start = Instant::now();
    let _ = cat_async_to_string(files).await;
    let async_dur = start.elapsed();

    (sync_dur, async_dur)
}

// === Tests ===
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[test]
    fn test_cat_sync_single_file() {
        let path = "test_sync.txt";
        let content = "Line1\r\nLine2";
        std::fs::write(path, content).unwrap();

        let output = cat(vec![path]).unwrap();
        assert_eq!(output, "Line1\nLine2\n");

        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn test_cat_async_to_string_file() {
        let path = "test_async.txt";
        let content = "Hello\r\nAsync";
        fs::write(path, content).await.unwrap();

        let output = cat_async_to_string(vec![path]).await.unwrap();
        assert_eq!(output, "Hello\nAsync\n");

        fs::remove_file(path).await.unwrap();
    }

    #[tokio::test]
    async fn test_cat_async_stream_file() {
        let path = "test_stream.txt";
        let content = "S1\r\nS2\nS3";
        fs::write(path, content).await.unwrap();

        let mut stream = cat_async(vec![path]).await;
        let mut collected = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    collected.push_str(&String::from_utf8_lossy(&bytes));
                }
                Err(e) => panic!("Stream failed: {:?}", e),
            }
        }

        assert_eq!(collected, "S1\nS2\nS3\n");
        fs::remove_file(path).await.unwrap();
    }

    #[tokio::test]
    async fn test_benchmark_runs() {
        let path = "bench.txt";
        let content = "Benchmark line\n".repeat(500);
        fs::write(path, content).await.unwrap();

        let (_sync, _async) = benchmark_cat_sync_vs_async(vec![path]).await;

        fs::remove_file(path).await.unwrap();
    }
}
