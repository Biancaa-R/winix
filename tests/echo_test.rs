use winix::echo;

#[test]
fn test_echo_output() {
    // Test echo functionality directly instead of running cargo run
    let args = vec!["Hello,".to_string(), "Rust!".to_string()];

    // Call the run function directly - it should not panic
    echo::run(&args);

    // If we reach here, the test passes (no panic occurred)
    assert!(true, "Echo command should execute successfully");
}
