#[cfg(test)]
mod tests {
    use winix::{
        cat::{benchmark_cat_sync_vs_async, cat_async_to_string},
        grep::{grep_async_to_string, grep_sync},
        head::{head_async_to_string, head_sync},
        pipeline::{CatGrepPipeline, CatHeadPipeline, execute_pipeline},
        tail::{tail_async_to_string, tail_sync},
    };

    #[tokio::test]
    async fn test_grep_async_basic() {
        let file_path = "test_grep_basic.txt";
        let content = "hello world\nthis is a test\nhello again\nbye world";

        tokio::fs::write(file_path, content).await.unwrap();

        let result = grep_async_to_string("hello", vec![file_path])
            .await
            .unwrap();
        assert!(result.contains("hello world"));
        assert!(result.contains("hello again"));
        assert!(!result.contains("bye world"));

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_head_async_basic() {
        let file_path = "test_head_basic.txt";
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

    #[tokio::test]
    async fn test_tail_async_basic() {
        let file_path = "test_tail_basic.txt";
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

    #[tokio::test]
    async fn test_cat_grep_pipeline() {
        let file_path = "test_pipeline_cat_grep.txt";
        let content = "hello world\nthis is a test\nhello again\nbye world";

        tokio::fs::write(file_path, content).await.unwrap();

        let pipeline = CatGrepPipeline::new(vec![file_path.to_string()], "hello".to_string());

        let result = execute_pipeline(pipeline).await.unwrap();
        assert!(result.contains("hello world"));
        assert!(result.contains("hello again"));
        assert!(!result.contains("bye world"));

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_cat_head_pipeline() {
        let file_path = "test_pipeline_cat_head.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        tokio::fs::write(file_path, content).await.unwrap();

        let pipeline = CatHeadPipeline::new(vec![file_path.to_string()], 3);

        let result = execute_pipeline(pipeline).await.unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_benchmark_comparison() {
        let file_path = "test_benchmark.txt";
        let content = "Test content\n".repeat(1000); // Create a larger file

        tokio::fs::write(file_path, content).await.unwrap();

        // Benchmark cat
        let (cat_sync_duration, cat_async_duration) =
            benchmark_cat_sync_vs_async(vec![file_path]).await;

        println!("Cat Benchmark:");
        println!("  Sync: {:?}", cat_sync_duration);
        println!("  Async: {:?}", cat_async_duration);
        println!(
            "  Speedup: {:.2}x",
            cat_sync_duration.as_micros() as f64 / cat_async_duration.as_micros() as f64
        );

        // Benchmark grep
        let start = std::time::Instant::now();
        let _grep_sync_result = grep_sync("Test", vec![file_path]);
        let grep_sync_duration = start.elapsed();

        let start = std::time::Instant::now();
        let _grep_async_result = grep_async_to_string("Test", vec![file_path]).await;
        let grep_async_duration = start.elapsed();

        println!("Grep Benchmark:");
        println!("  Sync: {:?}", grep_sync_duration);
        println!("  Async: {:?}", grep_async_duration);
        println!(
            "  Speedup: {:.2}x",
            grep_sync_duration.as_micros() as f64 / grep_async_duration.as_micros() as f64
        );

        // Benchmark head
        let start = std::time::Instant::now();
        let _head_sync_result = head_sync(vec![file_path], 100);
        let head_sync_duration = start.elapsed();

        let start = std::time::Instant::now();
        let _head_async_result = head_async_to_string(vec![file_path], 100).await;
        let head_async_duration = start.elapsed();

        println!("Head Benchmark:");
        println!("  Sync: {:?}", head_sync_duration);
        println!("  Async: {:?}", head_async_duration);
        println!(
            "  Speedup: {:.2}x",
            head_sync_duration.as_micros() as f64 / head_async_duration.as_micros() as f64
        );

        // Clean up
        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test non-existent file
        let result = cat_async_to_string(vec!["nonexistent.txt"]).await;
        assert!(result.is_err());

        let result = grep_async_to_string("pattern", vec!["nonexistent.txt"]).await;
        assert!(result.is_err());

        let result = head_async_to_string(vec!["nonexistent.txt"], 10).await;
        assert!(result.is_err());

        let result = tail_async_to_string(vec!["nonexistent.txt"], 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_files() {
        let file_path = "test_empty.txt";
        tokio::fs::write(file_path, "").await.unwrap();

        // Test cat with empty file
        let result = cat_async_to_string(vec![file_path]).await.unwrap();
        assert_eq!(result, "");

        // Test head with empty file
        let result = head_async_to_string(vec![file_path], 10).await.unwrap();
        assert_eq!(result, "");

        // Test tail with empty file
        let result = tail_async_to_string(vec![file_path], 10).await.unwrap();
        assert_eq!(result, "");

        // Test grep with empty file
        let result = grep_async_to_string("pattern", vec![file_path])
            .await
            .unwrap();
        assert_eq!(result, "");

        tokio::fs::remove_file(file_path).await.unwrap();
    }
}
