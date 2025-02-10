use crate::{backend::Backend, bench::Bench, cli::ExperimentParams, result::ExperimentData};

pub fn run(backend: Backend, benched: Bench, params: &ExperimentParams) -> ExperimentData {
    match backend {
        #[cfg(feature = "backend_ndarray")]
        Backend::NdArray => {
            use burn::backend::{NdArray, ndarray::NdArrayDevice};

            let device = NdArrayDevice::Cpu;
            let benched = benched.get_benchable::<NdArray>(&device);
            benched.bench(params, &device)
        }
        #[cfg(feature = "backend_wgpu")]
        Backend::WgpuDefaultDevice => {
            use burn::backend::{Wgpu, wgpu::WgpuDevice};

            let device = WgpuDevice::DefaultDevice;
            let benched = benched.get_benchable::<Wgpu>(&device);
            benched.bench(params, &device)
        }

        #[cfg(feature = "backend_tch")]
        Backend::TchCpu => {
            use burn::backend::{LibTorch, libtorch::LibTorchDevice};

            let device = LibTorchDevice::Cpu;
            let benched = benched.get_benchable::<LibTorch>(&device);
            benched.bench(params, &device)
        }
        #[cfg(feature = "backend_tch")]
        Backend::TchCuda0 => {
            use burn::backend::{LibTorch, libtorch::LibTorchDevice};

            let device = LibTorchDevice::Cuda(0);
            let benched = benched.get_benchable::<LibTorch>(&device);
            benched.bench(params, &device)
        }
        #[cfg(feature = "backend_tch")]
        Backend::TchVulkan => {
            use burn::backend::{LibTorch, libtorch::LibTorchDevice};

            let device = LibTorchDevice::Vulkan;
            let benched = benched.get_benchable::<LibTorch>(&device);
            benched.bench(params, &device)
        }
    }
}
