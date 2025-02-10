use crate::{backend::Backend, bench::Bench};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        backend: Backend,
        bench: Bench,
        #[command(flatten)]
        params: ExperimentParams,
    },
}

#[derive(Debug, Clone, Args)]
pub struct ExperimentParams {
    #[arg(short, long)]
    pub batch_size: usize,
    #[arg(long, default_value_t = 0)]
    pub preheat: usize,
    #[arg(short, long, default_value_t = 5)]
    pub run_count: usize,
}
