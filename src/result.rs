use crate::cli::ExperimentParams;

#[derive(Debug)]
pub struct ExperimentData {
    pub params: ExperimentParams,
    pub results: Vec<RunData>,
}

#[derive(Debug)]
pub struct RunData {
    pub time_ns: u128,
}
