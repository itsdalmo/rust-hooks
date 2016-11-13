//! Git hooks written in Rust.

extern crate regex;

pub mod error;
pub mod utils;

pub use regex::Regex;
use std::env;
use std::process;

fn check_commit(file: &str) -> Result<bool, error::Error> {
    let branch = try!(utils::current_branch());
    let master = try!(utils::string_matches(&branch, "master"));

    // Not allowed to commit to master
    if master {
        println!("Current branch is master. Checkout a valid branch.");
        process::exit(1);
    }

    // Branch must contain JIRA-issue
    let issue = match utils::jira_issue(&branch) {
        Ok(issue) => issue,
        Err(_)    => {
            println!("Branch does not contain reference to a JIRA-issue.");
            process::exit(1);
        }
    };

    // Message should contain JIRA-issue
    let message = try!(utils::read_message(file));
    let issue_in_message = try!(utils::string_starts_with(&message, issue));
    if !issue_in_message {
        println!("Commit message should include a reference to the JIRA-issue ({}).", issue);
        process::exit(1);
    }

    // Check complete
    Ok(true)
}

fn check_push() -> Result<bool, error::Error> {
    let branch = try!(utils::current_branch());
    let issue = match utils::jira_issue(&branch) {
        Ok(issue) => issue,
        Err(_)    => {
            println!("Branch does not contain reference to a JIRA-issue.");
            process::exit(1);
        }
    };

    let (local, remote) = try!(utils::get_refs());

    // Remote and local should have same name
    if local != remote {
        println!("Remote branch must have the same name as the local.");
        process::exit(1);
    }

    // Branches must also contain the JIRA issue
    let issue_in_branch = try!(utils::string_starts_with(&remote, &issue));
    if !issue_in_branch {
        println!("Remote branch must contain the JIRA-issue ({}).", issue);
        process::exit(1);
    }

    Ok(true)
}

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let hook = match utils::base_name(&args[0]) {
        Ok(v)  => v,
        Err(e) => {
            println!("Failed to check hook: {}", e);
            process::exit(1);
        },
    };

    match hook {
        "commit-msg" => {
            match check_commit(&args[1]) {
                Ok (_) => {
                },
                Err(e) => {
                    println!("Failed to check commit: {}", e);
                    process::exit(1);
                }
            }
        },
        "pre-push"   => {
            match check_push() {
                Ok (_) => {
                },
                Err(e) => {
                    println!("Failed to check push: {}", e);
                    process::exit(1);
                }
            }
        }
        _            => {
            println!("Unsupported hook.");
            process::exit(1);
        }
    }
}

