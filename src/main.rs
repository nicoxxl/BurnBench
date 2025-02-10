use std::u128;

use burn_bench::{
    backend::Backend,
    bench::Bench,
    cli::{Cli, Commands, ExperimentParams},
    run::run,
};
use clap::Parser as _;

const VERT: &'static str = "┊";

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Run {
            backend,
            bench,
            params,
        } => {
            run_bench(backend, bench, &params);
        }
    }
}

fn run_bench(backend: Backend, benched: Bench, params: &ExperimentParams) {
    println!("Running ...");
    let r = run(backend, benched, &params);
    println!(
        "Preheat : {}, batch size {}, run {}",
        r.params.preheat, r.params.batch_size, r.params.run_count
    );
    let mut min_ns = u128::MAX;
    let mut max_ns = u128::MIN;
    let mut avg_ns = 0;

    for run in &r.results {
        min_ns = min_ns.min(run.time_ns);
        max_ns = max_ns.max(run.time_ns);
        avg_ns += run.time_ns;
    }

    let avg_ns = (avg_ns as f64) / (r.results.len() as f64);

    for (i, run) in r.results.iter().enumerate() {
        let t_ms = run.time_ns as f64 / 1e6;

        println!("run {:2} {VERT} {:5.2} ms", i, t_ms);
    }
    println!("Average {:5.2} ms", avg_ns / 1e6);
}
