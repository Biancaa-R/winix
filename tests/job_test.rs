#![cfg(target_os = "windows")]

use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use winix::job::Job;

fn spawn_sleep_process() -> std::process::Child {
    // Use PowerShell's Start-Sleep for a lightweight long-running child
    Command::new("powershell")
        .args(&["-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to spawn sleep process")
}

#[test]
fn test_create_and_assign_process_to_job_and_terminate() {
    // Create job
    let job = Job::create().expect("Failed to create Job");

    // Spawn a child process
    let mut child = spawn_sleep_process();
    let pid = child.id();
    println!("Spawned child PID: {}", pid);

    // Assign child to job
    job.assign(pid).expect("Failed to assign process to job");

    // Give OS a moment
    sleep(Duration::from_millis(100));

    // Terminate all processes in job. This should kill the child.
    job.terminate(1).expect("Failed to terminate job");

    // Wait briefly and check child
    sleep(Duration::from_millis(200));

    match child.try_wait() {
        Ok(Some(status)) => {
            // Child has exited; success
            println!("Child exited with status: {:?}", status);
            assert!(true);
        }
        Ok(None) => {
            // Still running -> fail and attempt cleanup
            let _ = child.kill();
            panic!("Child still running after TerminateJobObject");
        }
        Err(e) => {
            panic!("Error checking child process: {}", e);
        }
    }
}

#[test]
fn test_assign_invalid_pid_fails() {
    let job = Job::create().expect("Failed to create Job");
    // Use an unlikely PID value
    let bad_pid: u32 = 999_999_999;
    let res = job.assign(bad_pid);
    assert!(res.is_err(), "Expected assigning invalid PID to fail");
}
