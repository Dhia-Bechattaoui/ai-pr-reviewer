use ai_pr_reviewer::{parse_unified_diff, GitProvider, LocalGitProvider};
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "ai-pr-reviewer")]
#[command(
    author,
    version,
    about = "AI PR Reviewer (AI-Debt Governor) intercepts low-quality AI-generated code diffs.",
    long_about = None
)]
struct Cli {
    /// Intercept staged/cached Git buffers instead of unstaged modifications
    #[arg(short, long, default_value_t = false)]
    staged: bool,

    /// Optional explicit raw diff string to parse directly instead of calling local Git binary
    #[arg(short, long)]
    raw_diff: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let diff_content = if let Some(raw) = cli.raw_diff {
        raw
    } else {
        let provider = LocalGitProvider;
        let res = if cli.staged {
            provider.get_staged_diff()
        } else {
            provider.get_unstaged_diff()
        };

        match res {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Operational Diagnostic Error: Failed to fetch Git diff: {}", err);
                return ExitCode::FAILURE;
            }
        }
    };

    match parse_unified_diff(&diff_content) {
        Ok(changed_files) => {
            // Guarantee deterministic output streams formatted as valid JSON objects
            match serde_json::to_string_pretty(&changed_files) {
                Ok(json_output) => {
                    println!("{}", json_output);
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("Serialization Error: Failed to format output JSON: {}", err);
                    ExitCode::FAILURE
                }
            }
        }
        Err(err) => {
            eprintln!("Operational Diagnostic Error: Failed to parse Unified Diff: {}", err);
            ExitCode::FAILURE
        }
    }
}
