# GitHub Service Rust Library

A Rust library designed to simplify interactions with the GitHub API, specifically focusing on repository content management. This library provides a convenient interface for common operations such as reading, listing, creating, updating, and deleting files within a GitHub repository.

## Features

- **File and Directory Operations**:
  - **`get_content_items`**: Retrieve a list of `Content` items (files, directories) from a specified path.
  - **`note_exists`**: Check if a file or directory exists at a given path.
  - **`create_file`**: Create a new file with specified content.
  - **`update_file`**: Update an existing file's content using its SHA.
  - **`delete_file`**: Delete a file using its SHA.
- **SHA Management**:
  - **`get_sha`**: Helper function to quickly retrieve the SHA of a file at a given path.
- **Robust Error Handling**:
  - Custom `GitHubServiceError` enum for handling API errors, race conditions, and other issues gracefully.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
github-service = "25.7.1"
```

## Usage

First, initialize the `GitHubService` with your GitHub personal access token, repository owner, and repository name.

```rust
use github_service::GitHubService;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN environment variable not set");
    let owner = "your_github_owner".to_string();
    let repo = "your_github_repo".to_string();

    let service = GitHubService::new(token, owner, repo);

    // --- Example: Create a new file ---
    let path = "example-dir/my-new-file.md";
    let content = "# Hello, World!\n\nThis is a test file.";
    let create_message = "feat: Add test file";

    if !service.note_exists(path).await? {
        match service.create_file(path, create_message, content).await {
            Ok(_) => println!("Successfully created file at '{}'", path),
            Err(e) => eprintln!("Failed to create file: {}", e),
        }
    } else {
        println!("File already exists at '{}'", path);
    }

    // --- Example: Read directory contents ---
    println!("\nListing contents of 'example-dir/':");
    match service.get_content_items("example-dir").await {
        Ok(items) => {
            for item in items {
                println!("- {} (type: {}, sha: {})", item.name, item.r#type, item.sha);
            }
        },
        Err(e) => eprintln!("Failed to list directory: {}", e),
    }

    // --- Example: Update the file ---
    let update_message = "docs: Update test file content";
    let updated_content = "# Hello, Universe!\n\nThis file has been updated.";

    // To update a file, you need its SHA
    if let Ok(sha) = service.get_sha(path).await {
        match service.update_file(path, update_message, updated_content, &sha).await {
            Ok(_) => println!("\nSuccessfully updated file at '{}'", path),
            Err(e) => eprintln!("Failed to update file: {}", e),
        }
    }

    // --- Example: Delete the file ---
    let delete_message = "refactor: Remove test file";

    // To delete a file, you also need its SHA
    if let Ok(sha) = service.get_sha(path).await {
        match service.delete_file(path, delete_message, &sha).await {
            Ok(_) => println!("\nSuccessfully deleted file at '{}'", path),
            Err(e) => eprintln!("Failed to delete file: {}", e),
        }
    }

    Ok(())
}
```

**Note**: Ensure you have a GitHub personal access token with appropriate permissions set as an environment variable named `GITHUB_TOKEN`.

## Building and Testing

To build the library:

```bash
cargo build
```

To run tests:

```bash
cargo test
```
