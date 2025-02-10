use crate::{
    cli::ExperimentParams,
    result::{ExperimentData, RunData},
};
use burn::prelude::Backend;
use clap::ValueEnum;
use dqn_nature_conv::ModelConfig;
use std::time::Instant;

pub mod dqn_nature_conv;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq, Hash)]
pub enum Bench {
    DqnNatureConv,
    
}

impl Bench {
    pub fn get_benchable<B: Backend>(&self, device: &B::Device) -> Box<dyn BenchModel<B>> {
        match self {
            Bench::DqnNatureConv => Box::new(ModelConfig::new().init(device)),
        }
    }
}

pub trait BenchableModel<B: Backend> {
    type Input;
    fn make_input(batch_size: usize, device: &B::Device) -> Self::Input;
    fn run(&self, input: &Self::Input);
}

pub trait BenchModel<B: Backend> {
    fn bench(&self, params: &ExperimentParams, device: &B::Device) -> ExperimentData;
}

impl<B: Backend, T: BenchableModel<B>> BenchModel<B> for T {
    fn bench(&self, params: &ExperimentParams, device: &B::Device) -> ExperimentData {
        let input = T::make_input(params.batch_size, device);
        let mut run_data = Vec::with_capacity(params.run_count);

        for _ in 0..params.preheat {
            self.run(&input);
        }
        for _ in 0..params.run_count {
            let start = Instant::now();
            self.run(&input);
            let end = Instant::now();
            run_data.push(RunData {
                time_ns: (end - start).as_nanos(),
            });
        }
        return ExperimentData {
            params: params.clone(),
            results: run_data,
        };
    }
}
