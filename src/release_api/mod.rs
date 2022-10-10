use crate::cli::RepositoryAddress;

pub trait ReleaseApiHandler {
    fn list(&mut self, repo: RepositoryAddress);
}
