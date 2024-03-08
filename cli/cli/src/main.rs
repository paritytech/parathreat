// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{App, Arg};
use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, ErrorKind};
use std::process::{Command, Stdio};

fn install_cargo_nextest_if_needed() {
    // Attempt to check if cargo-nextest is already installed
    let check_nextest_installed = Command::new("cargo")
        .arg("nextest")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match check_nextest_installed {
        Ok(status) if status.success() => {}
        _ => {
            // cargo-nextest not found, attempt to install
            println!("Dependency cargo-nextest not found, attempting to install...");
            let install_nextest = Command::new("cargo")
                .arg("install")
                .arg("cargo-nextest")
                .status();

            match install_nextest {
                Ok(status) if status.success() => {}
                _ => {
                    panic!("Failed to install cargo-nextest. Please install it manually first.");
                }
            }
        }
    }
}

#[allow(clippy::single_match)]
fn valid_threats(mission: &str, results: String) -> (u8, u8, u8) {
    let mut valid_threats = 0; // exploit vector still working
    let mut invalid_threats = 0; // exploit doesn't work anymore
    let mut works_tracking = HashMap::new();
    let mut fails_tracking = HashMap::new();
    match mission {
        "casino" => {
            for line in results.lines() {
                let line = line.trim();
                let mut parts: Vec<&str> = line.split("::").collect();

                if parts.len() > 1
                    && (line.starts_with("PASS")
                        || line.starts_with("FAIL")
                        || line.starts_with("SIGABRT"))
                {
                    if let Some(result) = parts.pop() {
                        if result.ends_with("fails")
                            && !fails_tracking.contains_key(&result.to_string())
                        {
                            fails_tracking.insert(result.to_string(), true);
                            if !line.starts_with("PASS") {
                                valid_threats += 1;
                            }
                        } else if result.ends_with("works")
                            && !works_tracking.contains_key(&result.to_string())
                        {
                            works_tracking.insert(result.to_string(), true);
                            if !line.starts_with("PASS") {
                                invalid_threats += 1;
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }
    (
        valid_threats,
        (works_tracking.len() as u8)
            .checked_sub(invalid_threats)
            .unwrap(),
        works_tracking.len() as u8,
    )
}

/// Checks the specified mission.
fn check(mission: &str, team: &str) -> io::Result<()> {
    install_cargo_nextest_if_needed();

    println!("Checking {} mission...", mission);

    let package = match team {
        "red" => "threats",
        _ => "parathreat-verify",
    };

    let tests = format!("{}::", mission);
    let output = Command::new("cargo")
        .args(["nextest", "run", &tests, "-p", package, "--release"])
        .current_dir("./") // Adjust this path to your workspace root
        .output()
        .map_err(|_| io::Error::new(ErrorKind::Other, "Failed to checks".to_string()))?;

    let results = String::from_utf8_lossy(&output.stderr).to_string();
    let (valid_threats, invalid_threats, threats) = valid_threats(mission, results.clone());

    match team {
        "red" => {
            println!("Threats working: {}/7", threats);
        }
        _ => {
            if valid_threats > 0 || invalid_threats > 0 {
                let threats = max(valid_threats, invalid_threats);
                println!(
                    "Ops! {} vulnerabilities have been exploited in the Casino Parachain! Try again...",
                    threats
                );
            } else {
                println!("Success! You were able to mitigate all threats in the Casino Parachain!");
            }
        }
    }

    Ok(())
}

fn main() {
    let matches = App::new("Parathreat")
        .version("1.0")
        .about("Executes Parathreat threat scenarios")
        .arg(
            Arg::new("MISSION")
                .help("The mission to check")
                .default_value("casino")
                .index(1),
        )
        .arg(
            Arg::new("TEAM")
                .help("The team/mode in which you want to play")
                .short('t')
                .long("team")
                .default_value("blue"),
        )
        .get_matches();

    let mission = matches.value_of("MISSION").unwrap();
    let mut team = matches.value_of("TEAM").unwrap();

    if !std::path::Path::new("cli/verify").is_dir() {
        println!("Verify folder not found, running in red team mode.");
        team = "red";
    }

    if let Err(e) = check(mission, team) {
        eprintln!("Error: {}", e);
    }
}
