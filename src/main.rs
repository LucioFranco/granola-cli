mod error;
mod models;
mod cache;
mod commands;

use clap::{Parser, Subcommand};
use error::{GranolaError, Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "granola")]
#[command(about = "Query Granola meeting data", long_about = None)]
#[command(after_help = "For AI assistants: Run `granola workflow` for usage patterns, output schemas, and best practices.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to cache file (overrides env var and default)
    #[arg(long, global = true)]
    cache_path: Option<PathBuf>,

    /// Output errors as JSON to stdout (default: stderr)
    #[arg(long, global = true)]
    json_errors: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Search meetings by query
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long, default_value = "30")]
        limit: usize,
    },

    /// Get meeting metadata
    Details {
        /// Meeting ID
        meeting_id: String,
    },

    /// Get meeting transcript
    Transcript {
        /// Meeting ID
        meeting_id: String,
    },

    /// Get meeting notes/documents
    Documents {
        /// Meeting ID
        meeting_id: String,
    },

    /// Show usage patterns (for AI assistants)
    Workflow,
}

fn main() {
    let cli = Cli::parse();
    let json_errors = cli.json_errors;

    let result = run(cli);

    match result {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(0);
        }
        Err(e) => {
            if json_errors {
                println!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
            } else {
                eprintln!("{}", e);
            }
            std::process::exit(e.exit_code());
        }
    }
}

fn run(cli: Cli) -> Result<String> {
    // Handle workflow command separately (no cache needed)
    if matches!(cli.command, Commands::Workflow) {
        return Ok(commands::get_workflow_guide());
    }

    // Load cache for other commands
    let cache_path = cache::resolve_cache_path(cli.cache_path);
    let cache = cache::load_cache(&cache_path)?;

    // Dispatch to appropriate command
    match cli.command {
        Commands::Search { query, limit } => {
            let output = commands::search_meetings(&query, limit, &cache)?;
            Ok(serde_json::to_string_pretty(&output)?)
        }
        Commands::Details { meeting_id } => {
            let output = commands::get_meeting_details(&meeting_id, &cache)?;
            Ok(serde_json::to_string_pretty(&output)?)
        }
        Commands::Transcript { meeting_id } => {
            let output = commands::get_transcript(&meeting_id, &cache)?;
            Ok(serde_json::to_string_pretty(&output)?)
        }
        Commands::Documents { meeting_id } => {
            let output = commands::get_documents(&meeting_id, &cache)?;
            Ok(serde_json::to_string_pretty(&output)?)
        }
        Commands::Workflow => unreachable!(),
    }
}
