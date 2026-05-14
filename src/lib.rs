//! Library crate exposing reusable modules for the AI PR Reviewer (AI-Debt Governor).
//! Fully decoupled architecture isolating parsing layers from evaluation engines.

pub mod git;

pub use git::{
    parse_unified_diff, ChangedFile, GitError, GitProvider, Hunk, LineChunk, LineType,
    LocalGitProvider,
};
