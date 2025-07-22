use anyhow::{Context, Result};
use octocrab::models::repos::Content;
use octocrab::Octocrab;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum GitHubServiceError {
    #[error("Note already exists")]
    NoteAlreadyExists,
    #[error("GitHub API error: {0}")]
    Octocrab(String),
    #[error("An internal error occurred: {0}")]
    Anyhow(String),
}

impl From<anyhow::Error> for GitHubServiceError {
    fn from(err: anyhow::Error) -> Self {
        GitHubServiceError::Anyhow(err.to_string())
    }
}

impl From<octocrab::Error> for GitHubServiceError {
    fn from(err: octocrab::Error) -> Self {
        GitHubServiceError::Octocrab(err.to_string())
    }
}

pub struct GitHubService {
    client: Octocrab,
    repo_owner: String,
    repo_name: String,
}

impl GitHubService {
    pub fn new(token: String, repo_owner: String, repo_name: String) -> Self {
        let client = Octocrab::builder().personal_token(token).build().unwrap();
        Self {
            client,
            repo_owner,
            repo_name,
        }
    }

    pub async fn get_content_items(&self, path: &str) -> Result<Vec<Content>> {
        Ok(self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(path)
            .send()
            .await?
            .items)
    }

    pub async fn note_exists(&self, path: &str) -> Result<bool> {
        match self.client.repos(&self.repo_owner, &self.repo_name).get_content().path(path).send().await {
            Ok(_) => Ok(true),
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn create_file(&self, path: &str, message: &str, content: &str) -> Result<(), GitHubServiceError> {
        match self.client.repos(&self.repo_owner, &self.repo_name).create_file(path, message, content).send().await {
            Ok(_) => Ok(()),
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 422 => Ok(()), // Race condition
            Err(e) => Err(e.into()),
        }
    }

    pub async fn update_file(&self, path: &str, message: &str, content: &str, sha: &str) -> Result<()> {
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .update_file(path, message, content, sha)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_file(&self, path: &str, message: &str, sha: &str) -> Result<()> {
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .delete_file(path, message, sha)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_sha(&self, path: &str) -> Result<String> {
        let content = self.get_content_items(path).await?;
        let sha = content.first().context("No content found")?.sha.clone();
        Ok(sha)
    }
}