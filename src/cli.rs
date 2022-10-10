use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List { repo: RepositoryAddress },
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RepositoryAddress {
    pub owner: String,
    pub repo: String,
}

impl std::str::FromStr for RepositoryAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || format!("Expected a handle of the form <owner>/<repo> ; found {s}");
        if !s.is_ascii() {
            return Err(err());
        }
        let (owner, repo) = s.split_once('/').ok_or_else(err)?;
        if owner.is_empty() {
            return Err(err());
        }
        if repo.is_empty() || repo.split_once('/').is_some() {
            return Err(err());
        }
        Ok(Self {
            owner: owner.into(),
            repo: repo.into(),
        })
    }
}
