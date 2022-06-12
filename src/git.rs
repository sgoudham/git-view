use core::fmt;
use mockall::automock;
use std::process::{Command, Output};

use crate::error::AppError;

pub(crate) enum Git<'a> {
    IsValidRepository,
    LocalBranch,
    DefaultRemote,
    TrackedRemote(&'a str),
    UpstreamBranch(&'a str),
    IsValidRemote(&'a str),
}

#[derive(Debug)]
pub(crate) enum Domain {
    Github(String),
    BitBucket(String),
}

#[derive(Debug)]
pub(crate) struct Url {
    pub(crate) protocol: String,
    pub(crate) domain: Domain,
    pub(crate) path: String,
}

pub(crate) enum GitOutput {
    Ok(String),
    Err(String),
}

#[automock]
pub(crate) trait GitCommand {
    fn execute(&self) -> Result<GitOutput, AppError>;
}

impl<'a> Git<'a> {
    fn command(&self) -> Result<Output, std::io::Error> {
        match *self {
            Git::IsValidRepository => Command::new("git")
                .arg("rev-parse")
                .arg("--is-inside-work-tree")
                .output(),
            Git::LocalBranch => Command::new("git")
                .arg("symbolic-ref")
                .arg("-q")
                .arg("--short")
                .arg("HEAD")
                .output(),
            Git::DefaultRemote => Command::new("git")
                .arg("config")
                .arg("open.default.remote")
                .output(),
            Git::TrackedRemote(branch) => Command::new("git")
                .arg("config")
                .arg(format!("branch.{}.remote", branch))
                .output(),
            Git::UpstreamBranch(branch) => Command::new("git")
                .arg("config")
                .arg(format!("branch.{}.merge", branch))
                .output(),
            Git::IsValidRemote(remote) => Command::new("git")
                .arg("ls-remote")
                .arg("--get-url")
                .arg(remote)
                .output(),
        }
    }

    fn trim(&self, bytes: &[u8]) -> Result<String, AppError> {
        let mut utf_8_string = String::from(std::str::from_utf8(bytes)?.trim());

        if utf_8_string.ends_with('\n') {
            utf_8_string.pop();
            if utf_8_string.ends_with('\r') {
                utf_8_string.pop();
            }
        }

        Ok(utf_8_string)
    }
}

impl<'a> GitCommand for Git<'a> {
    fn execute(&self) -> Result<GitOutput, AppError> {
        let command = self.command()?;
        if command.status.success() {
            Ok(GitOutput::Ok(self.trim(&command.stdout)?))
        } else {
            Ok(GitOutput::Err(self.trim(&command.stderr)?))
        }
    }
}

impl Url {
    pub(crate) fn new(protocol: &str, domain: Domain, path: &str) -> Self {
        Self {
            protocol: protocol.to_owned(),
            domain,
            path: path.to_owned(),
        }
    }
}

impl Domain {
    pub(crate) fn from_str(s: &str) -> Self {
        if s == "bitbucket.org" {
            Domain::BitBucket(s.to_owned())
        } else {
            Domain::Github(s.to_owned())
        }
    }
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Domain::Github(str) | Domain::BitBucket(str) => write!(f, "{}", str),
        }
    }
}
