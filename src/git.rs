use std::{
    borrow::Cow,
    process::{Command, Output},
};

use crate::error::AppError;
#[cfg(test)]
use mockall::automock;

#[derive(Debug, PartialEq)]
pub(crate) enum Local<'a> {
    Branch(Cow<'a, str>),
    NotBranch,
}

#[derive(Debug)]
pub(crate) struct Url {
    pub(crate) protocol: String,
    pub(crate) domain: String,
    pub(crate) path: String,
}

#[derive(Default)]
pub struct Git;

pub(crate) enum GitCommand<'a> {
    IsValidRepository,
    LocalBranch,
    DefaultRemote,
    TrackedRemote(&'a str),
    UpstreamBranch(&'a str),
    DefaultBranch(&'a str),
    IsValidRemote(&'a str),
    CurrentTag,
    CurrentCommit,
    CurrentWorkingDirectory,
}

pub enum GitOutput {
    Ok(String),
    Err(String),
}

#[cfg_attr(test, automock)]
pub trait GitTrait {
    fn is_valid_repository(&self) -> Result<GitOutput, AppError>;
    fn get_local_branch(&self) -> Result<GitOutput, AppError>;
    fn get_default_remote(&self) -> Result<GitOutput, AppError>;
    fn get_tracked_remote(&self, tracked: &str) -> Result<GitOutput, AppError>;
    fn get_upstream_branch(&self, branch: &str) -> Result<GitOutput, AppError>;
    fn get_default_branch(&self, remote: &str) -> Result<GitOutput, AppError>;
    fn is_valid_remote(&self, remote: &str) -> Result<GitOutput, AppError>;
    fn get_current_tag(&self) -> Result<GitOutput, AppError>;
    fn get_current_commit(&self) -> Result<GitOutput, AppError>;
    fn get_current_working_directory(&self) -> Result<GitOutput, AppError>;
}

impl GitTrait for Git {
    fn is_valid_repository(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::IsValidRepository)?)
    }

    fn get_local_branch(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::LocalBranch)?)
    }

    fn get_default_remote(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::DefaultRemote)?)
    }

    fn get_tracked_remote(&self, tracked: &str) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::TrackedRemote(tracked))?)
    }

    fn get_upstream_branch(&self, branch: &str) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::UpstreamBranch(branch))?)
    }

    fn get_default_branch(&self, remote: &str) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::DefaultBranch(remote))?)
    }

    fn is_valid_remote(&self, remote: &str) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::IsValidRemote(remote))?)
    }

    fn get_current_tag(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::CurrentTag)?)
    }

    fn get_current_commit(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::CurrentCommit)?)
    }

    fn get_current_working_directory(&self) -> Result<GitOutput, AppError> {
        execute(command(GitCommand::CurrentWorkingDirectory)?)
    }
}

fn command(git_command: GitCommand) -> Result<Output, std::io::Error> {
    match git_command {
        GitCommand::IsValidRepository => Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output(),
        GitCommand::LocalBranch => Command::new("git")
            .arg("symbolic-ref")
            .arg("-q")
            .arg("--short")
            .arg("HEAD")
            .output(),
        GitCommand::DefaultRemote => Command::new("git")
            .arg("config")
            .arg("open.default.remote")
            .output(),
        GitCommand::TrackedRemote(branch) => Command::new("git")
            .arg("config")
            .arg(format!("branch.{}.remote", branch))
            .output(),
        GitCommand::UpstreamBranch(branch) => Command::new("git")
            .arg("config")
            .arg(format!("branch.{}.merge", branch))
            .output(),
        GitCommand::DefaultBranch(remote) => Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg(format!("{}/HEAD", remote))
            .output(),
        GitCommand::IsValidRemote(remote) => Command::new("git")
            .arg("ls-remote")
            .arg("--get-url")
            .arg(remote)
            .output(),
        GitCommand::CurrentTag => Command::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--exact-match")
            .output(),
        GitCommand::CurrentCommit => Command::new("git").arg("rev-parse").arg("HEAD").output(),
        GitCommand::CurrentWorkingDirectory => Command::new("git")
            .arg("rev-parse")
            .arg("--show-prefix")
            .output(),
    }
}

fn execute(output: Output) -> Result<GitOutput, AppError> {
    if output.status.success() {
        Ok(GitOutput::Ok(trim(&output.stdout)?))
    } else {
        Ok(GitOutput::Err(trim(&output.stderr)?))
    }
}

fn trim(bytes: &[u8]) -> Result<String, AppError> {
    let mut utf_8_string = String::from(std::str::from_utf8(bytes)?.trim());

    if utf_8_string.ends_with('\n') {
        utf_8_string.pop();
        if utf_8_string.ends_with('\r') {
            utf_8_string.pop();
        }
    }

    Ok(utf_8_string)
}

impl Url {
    pub(crate) fn new(protocol: &str, domain: &str, path: &str) -> Self {
        Self {
            protocol: protocol.into(),
            domain: domain.into(),
            path: path.into(),
        }
    }
}
