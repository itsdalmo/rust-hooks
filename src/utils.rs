extern crate regex;

use regex::Regex;
use std::{io, result};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use error;

pub type Result<T> = result::Result<T, error::Error>;

// Subprocess to find the current branch -------------------------------------
pub fn current_branch() -> Result<String> {
    let cmd = try!(Command::new("git")
        .args(&["symbolic-ref", "--short", "HEAD"])
        .output());
    let branch = try!(String::from_utf8(cmd.stdout));
    Ok(branch.trim_right().to_string())
}

#[test]
fn test_current_branch() {
}

// Extracts the hook-type from argument --------------------------------------
pub fn base_name(path: &str) -> Result<&str> {
    let parts: Vec<&str> = path.split("/").collect();
    match parts.last() {
        Some(v)  => Ok(v),
        None     => Err(error::Error::Missing("base_name")),
    }
}

#[test]
fn test_base_name() {
    let callee = ".git/hooks/commit-msg";
    match base_name(callee) {
        Ok(v)  => assert_eq!(v, "commit-msg"),
        Err(e) => assert!(false),
    };
}

// Checks whether a string contains a Jira issue (e.g. DA-123) ---------------
pub fn jira_issue(branch: &str) -> Result<&str> {
    let check = try!(Regex::new(r"^(\w{2,2}-\d+)"));
    match check.captures(branch) {
        Some(issue) => {
            match issue.at(1) {
                Some(i) => Ok(i),
                None    => Err(error::Error::Missing("jira_issue")),
            }
        },
        None        => Err(error::Error::Missing("jira_issue")),
    }
}

#[test]
fn test_jira_issue() {
    let issue = "DA-999_test_branch";
    match jira_issue(issue) {
        Ok(v)  => assert_eq!(v, "DA-999"),
        Err(e) => assert!(false),
    }
    match jira_issue("no_issue") {
        Ok(v)  => assert!(false),
        Err(e) => assert!(true),
    }
}

// Checks whether 's' is equal to 'c' ----------------------------------------
pub fn string_matches(s: &str, c: &str) -> Result<bool> {
    let check = try!(Regex::new(&format!("^{}$", c)));
    let result = check.is_match(&s);
    Ok(result)
}

#[test]
fn test_string_matches() {
    let s = "master";
    match string_matches(s, "master") {
        Ok(v)  => assert!(v),
        Err(e) => assert!(false),
    }
//    match string_matches(s, "MASTER") {
//        Ok(v)  => assert!(v),
//        Err(e) => assert!(false),
//    }
    match string_matches(s, "not") {
        Ok(v)  => assert!(!v),
        Err(e) => assert!(false),
    }
}

// Checks whether 's' starts with 'c' ----------------------------------------
pub fn string_starts_with(s: &str, c: &str) -> Result<bool> {
    let check = try!(Regex::new(&format!("^{}.*$", c)));
    let result = check.is_match(&s);
    Ok(result)
}

#[test]
fn test_string_starts_with() {
    let s = "DA-999: Did stuff";
    match string_starts_with(s, "DA-999") {
        Ok(v)  => assert!(v),
        Err(v) => assert!(false),
    }
    match string_starts_with(s, "nope") {
        Ok(v)  => assert!(!v),
        Err(v) => assert!(false),
    }
}

// Reads stdin to find git push references -----------------------------------
fn parse_refs(s: &str) -> Result<(String, String)> {
    let refs: Vec<&str> = s.trim_right().split(" ").collect();
    if refs.len() < 3 {
        return Err(error::Error::Missing("parse_refs"));
    }
    let local  = try!(base_name(refs[0]));
    let remote = try!(base_name(refs[2]));
    let res: (String, String) = (local.to_string(), remote.to_string());
    Ok(res)
}

#[test]
fn test_parse_refs() {
    let s = "refs/heads/DA-890_lol 55d4499d1f96b014eed76b3603dd077bc6b51972 refs/heads/DA-890_lol 0000000000000000000000000000000000000000\n";
    match parse_refs(&s) {
        Ok(v)  => {
            let (local, remote) = v;
            assert_eq!(local, "DA-890_lol");
            assert_eq!(local, remote);

        },
        Err(e) => assert!(false),
    }
}

pub fn get_refs() -> Result<(String, String)> {
    let mut buffer = String::new();
    try!(io::stdin().read_to_string(&mut buffer));
    let res = try!(parse_refs(&buffer));
    Ok(res)
}

#[test]
fn test_push_refs() {
}

// Read a file to string -----------------------------------------------------
pub fn read_message(file: &str) -> Result<String> {
    let mut f = try!(File::open(file));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s.trim_right().to_string())
}

