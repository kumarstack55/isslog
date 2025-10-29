use chrono::Local;
use rand::distr::{Alphanumeric, SampleString};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let log_dir_path = "/var/log/isslog";

    let date_str = Local::now().format("%Y%m%d_%H%M%S_%Z").to_string();
    let userid = whoami::username();

    let mut rng = rand::rng();
    let random: String = Alphanumeric.sample_string(&mut rng, 10);

    let prefix = format!("{}_{}_{}", date_str, userid, random);
    let timing_path = format!("{}/{}_timing.txt", log_dir_path, prefix);
    let log_in_path = format!("{}/{}_log_in.bin", log_dir_path, prefix);
    let log_out_path = format!("{}/{}_log_out.bin", log_dir_path, prefix);

    if !Path::new(log_dir_path).exists() {
        if let Err(e) = fs::create_dir_all(log_dir_path) {
            eprintln!("Failed to create log dir: {}", e);
            exit(1);
        }
    }

    if env::var("SSH_ORIGINAL_COMMAND").is_ok() {
        eprintln!("This SSH service is for interactive use only.");
        eprintln!("Do not specify a command like `ssh host \"command arguments...\"`.");
        exit(1);
    }

    println!("NOTE: This SSH session is being recorded.");

    let status = Command::new("script")
        .args([
            "--quiet",
            "--flush",
            "--logging-format=advanced",
            &format!("--timing={}", timing_path),
            &format!("--log-in={}", log_in_path),
            &format!("--log-out={}", log_out_path),
            "--command=/bin/bash",
        ])
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("script exited with status {}", s),
        Err(e) => eprintln!("failed to execute script: {}", e),
    }
}
