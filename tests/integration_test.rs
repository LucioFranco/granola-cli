use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Query Granola meeting data"));
    assert!(stdout.contains("search"));
    assert!(stdout.contains("workflow"));
}

#[test]
fn test_workflow_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "workflow"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Granola CLI Workflow Guide"));
    assert!(stdout.contains("Token Optimization Tips"));
}

#[test]
fn test_cache_not_found_error() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "search",
            "test",
            "--cache-path",
            "/nonexistent/path.json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Cache file not found"));
}

#[test]
fn test_json_errors_flag() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "search",
            "test",
            "--cache-path",
            "/nonexistent/path.json",
            "--json-errors",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"error\""));
    assert!(stdout.contains("\"code\""));
    assert!(stdout.contains("cache_not_found"));
}
