use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Failed to execute git command: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git command failed with exit code {code}: {stderr}")]
    CommandFailed { code: i32, stderr: String },
    #[error("Failed to parse git diff output: {0}")]
    ParseError(String),
}

/// Represents a clean abstraction for fetching Git staging buffers/diffs.
/// Adheres to Architecture Rule 1: Decoupled Engines.
pub trait GitProvider {
    /// Retrieves the cached/staged diff buffer.
    fn get_staged_diff(&self) -> Result<String, GitError>;
    /// Retrieves the unstaged diff buffer.
    fn get_unstaged_diff(&self) -> Result<String, GitError>;
}

/// A standard production implementation interacting with local Git binary staging buffers.
pub struct LocalGitProvider;

impl GitProvider for LocalGitProvider {
    fn get_staged_diff(&self) -> Result<String, GitError> {
        let output = Command::new("git")
            .args(["diff", "--cached"])
            .output()?;

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(GitError::CommandFailed { code, stderr });
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn get_unstaged_diff(&self) -> Result<String, GitError> {
        let output = Command::new("git")
            .args(["diff"])
            .output()?;

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(GitError::CommandFailed { code, stderr });
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LineType {
    Context,
    Added,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineChunk {
    pub line_type: LineType,
    pub content: String,
    /// The precise line number in the target/modified file, if applicable.
    pub new_line_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub line_chunks: Vec<LineChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedFile {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub hunks: Vec<Hunk>,
}

/// Parses standard Unified Diff output into structured, isolated modification chunks.
pub fn parse_unified_diff(diff_str: &str) -> Result<Vec<ChangedFile>, GitError> {
    let mut files = Vec::new();
    let mut current_file: Option<ChangedFile> = None;
    let mut current_hunk: Option<Hunk> = None;
    let mut current_new_line = 0;

    for line in diff_str.lines() {
        if line.starts_with("diff --git") {
            // Push existing accumulated hunk and file if present
            if let Some(h) = current_hunk.take() {
                if let Some(ref mut f) = current_file {
                    f.hunks.push(h);
                }
            }
            if let Some(f) = current_file.take() {
                files.push(f);
            }

            current_file = Some(ChangedFile {
                old_path: None,
                new_path: None,
                hunks: Vec::new(),
            });
        } else if line.starts_with("--- ") {
            if let Some(ref mut f) = current_file {
                let path_str = line.trim_start_matches("--- ").trim();
                let clean_path = path_str.strip_prefix("a/").unwrap_or(path_str);
                if clean_path != "/dev/null" {
                    f.old_path = Some(clean_path.to_string());
                }
            }
        } else if line.starts_with("+++ ") {
            if let Some(ref mut f) = current_file {
                let path_str = line.trim_start_matches("+++ ").trim();
                let clean_path = path_str.strip_prefix("b/").unwrap_or(path_str);
                if clean_path != "/dev/null" {
                    f.new_path = Some(clean_path.to_string());
                }
            }
        } else if line.starts_with("@@ ") {
            // Push previous hunk into current file if present
            if let Some(h) = current_hunk.take() {
                if let Some(ref mut f) = current_file {
                    f.hunks.push(h);
                }
            }

            // Example header: @@ -1,3 +1,5 @@
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let old_range = parts[1].trim_start_matches('-');
                let new_range = parts[2].trim_start_matches('+');

                let (old_start, old_lines) = parse_range(old_range);
                let (new_start, new_lines) = parse_range(new_range);

                current_new_line = new_start;
                current_hunk = Some(Hunk {
                    old_start,
                    old_lines,
                    new_start,
                    new_lines,
                    line_chunks: Vec::new(),
                });
            }
        } else if let Some(ref mut hunk) = current_hunk {
            if let Some(stripped) = line.strip_prefix('+') {
                hunk.line_chunks.push(LineChunk {
                    line_type: LineType::Added,
                    content: stripped.to_string(),
                    new_line_number: Some(current_new_line),
                });
                current_new_line += 1;
            } else if let Some(stripped) = line.strip_prefix('-') {
                hunk.line_chunks.push(LineChunk {
                    line_type: LineType::Deleted,
                    content: stripped.to_string(),
                    new_line_number: None,
                });
            } else if let Some(stripped) = line.strip_prefix(' ') {
                hunk.line_chunks.push(LineChunk {
                    line_type: LineType::Context,
                    content: stripped.to_string(),
                    new_line_number: Some(current_new_line),
                });
                current_new_line += 1;
            } else if line.is_empty() {
                // Empty context line
                hunk.line_chunks.push(LineChunk {
                    line_type: LineType::Context,
                    content: String::new(),
                    new_line_number: Some(current_new_line),
                });
                current_new_line += 1;
            } else if line.starts_with('\\') {
                // \ No newline at end of file -> skip cleanly
            } else {
                // Unexpected prefix inside hunk, treat as context or skip
            }
        }
    }

    // Flush final dangling hunk/file buffers
    if let Some(h) = current_hunk.take() {
        if let Some(ref mut f) = current_file {
            f.hunks.push(h);
        }
    }
    if let Some(f) = current_file.take() {
        files.push(f);
    }

    Ok(files)
}

/// Helper function to parse chunk range elements like "1,3" or "1" cleanly.
fn parse_range(range_str: &str) -> (usize, usize) {
    if let Some((start_str, lines_str)) = range_str.split_once(',') {
        let start = start_str.parse().unwrap_or(0);
        let lines = lines_str.parse().unwrap_or(0);
        (start, lines)
    } else {
        let start = range_str.parse().unwrap_or(0);
        (start, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unified_diff_clean() {
        let sample_diff = "\
diff --git a/src/main.rs b/src/main.rs
index e69de29..d95f3ad 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
 fn main() {
-    println!(\"old\");
+    println!(\"new\");
+    // trailing comment
 }";
        let files = parse_unified_diff(sample_diff).expect("Failed to parse diff");
        assert_eq!(files.len(), 1);
        let file = &files[0];
        assert_eq!(file.old_path.as_deref(), Some("src/main.rs"));
        assert_eq!(file.new_path.as_deref(), Some("src/main.rs"));
        assert_eq!(file.hunks.len(), 1);

        let hunk = &file.hunks[0];
        assert_eq!(hunk.new_start, 1);
        assert_eq!(hunk.line_chunks.len(), 5);
        assert_eq!(hunk.line_chunks[0].line_type, LineType::Context);
        assert_eq!(hunk.line_chunks[1].line_type, LineType::Deleted);
        assert_eq!(hunk.line_chunks[2].line_type, LineType::Added);
        assert_eq!(hunk.line_chunks[2].new_line_number, Some(2));
        assert_eq!(hunk.line_chunks[3].line_type, LineType::Added);
        assert_eq!(hunk.line_chunks[3].new_line_number, Some(3));
        assert_eq!(hunk.line_chunks[4].line_type, LineType::Context);
        assert_eq!(hunk.line_chunks[4].new_line_number, Some(4));
    }
}
