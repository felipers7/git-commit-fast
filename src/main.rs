use std::env;
use std::process::Command;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a commit message.");
        return;
    }

    let commit_message = args[1..].join(" ");

    let output = Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg(&commit_message)
        .output()
        .expect("Failed to execute git command.");

    if output.status.success() {
        println!("Git commit successful.");
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Git commit failed: {}", error_message);
    }
}
