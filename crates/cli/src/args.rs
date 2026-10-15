use std::path::PathBuf;

use acvc_domain::Result;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Format {
    Json,
    Text,
}

#[derive(Parser, Debug)]
#[command(name = "acvc")]
pub struct Cli {
    #[arg(long, global = true)]
    pub repo: Option<PathBuf>,
    #[arg(long, global = true, value_enum, default_value_t = Format::Json)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Campaign {
        #[command(subcommand)]
        action: CampaignCommand,
    },
    Seed {
        #[arg(long)]
        campaign: Option<String>,
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        premise: String,
        #[arg(long = "assumption")]
        assumptions: Vec<String>,
        #[arg(long, default_value = "")]
        intent: String,
        #[arg(long, default_value = "agent")]
        agent: String,
    },
    Materialize {
        #[arg(long)]
        variant: String,
        #[arg(long)]
        into: PathBuf,
    },
    Status {
        #[arg(long)]
        workspace: Option<PathBuf>,
    },
    Record {
        #[arg(long)]
        workspace: Option<PathBuf>,
        #[arg(long, default_value = "mutate")]
        edge: String,
        #[arg(long)]
        premise: String,
        #[arg(long = "assumption")]
        assumptions: Vec<String>,
        #[arg(long)]
        revises: Option<String>,
        #[arg(long, default_value = "")]
        intent: String,
        #[arg(long, default_value = "agent")]
        agent: String,
        #[arg(long = "parent")]
        parents: Vec<String>,
    },
    Score {
        #[arg(long)]
        variant: Option<String>,
        #[arg(long = "metric", required = true)]
        metrics: Vec<String>,
    },
    Reject {
        #[arg(long)]
        variant: Option<String>,
        #[arg(long)]
        reason: String,
    },
    Compare {
        left: String,
        right: String,
    },
    FileDiff {
        left: String,
        right: String,
        #[arg(long)]
        path: String,
    },
    Frontier {
        #[arg(long)]
        campaign: Option<String>,
    },
    Recall {
        #[arg(long)]
        campaign: Option<String>,
        #[arg(long)]
        query: String,
    },
    Why {
        variant: Option<String>,
    },
    Backtrack {
        variant: String,
        #[arg(long)]
        into: Option<PathBuf>,
    },
    Pack,
    Serve {
        #[arg(long, default_value = "127.0.0.1:9419")]
        bind: String,
    },
    Push {
        #[arg(long)]
        to: String,
        #[arg(long)]
        variant: String,
        #[arg(long)]
        campaign: Option<String>,
    },
    Fetch {
        #[arg(long)]
        from: String,
        #[arg(long)]
        variant: String,
        #[arg(long)]
        campaign: Option<String>,
    },
    Promote {
        #[arg(long)]
        variant: String,
        #[arg(long)]
        dest: Option<PathBuf>,
        #[arg(long)]
        git: Option<PathBuf>,
    },
    Tui,
    Mcp,
    Web {
        #[arg(long, default_value = "127.0.0.1:9420")]
        bind: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CampaignCommand {
    Create {
        name: String,
        #[arg(long = "metric", required = true)]
        metrics: Vec<String>,
    },
    List,
}

pub fn parse() -> Result<Cli> {
    match Cli::try_parse() {
        Ok(cli) => Ok(cli),
        Err(err)
            if err.kind() == clap::error::ErrorKind::DisplayHelp
                || err.kind() == clap::error::ErrorKind::DisplayVersion =>
        {
            let _ = err.print();
            std::process::exit(0);
        }
        Err(err) => Err(acvc_domain::Error::new(err.to_string()).ctx("op", "parse")),
    }
}
