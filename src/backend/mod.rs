use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq, Hash)]
pub enum Backend {
    #[cfg(feature = "backend_ndarray")]
    NdArray,
    #[cfg(feature = "backend_wgpu")]
    WgpuDefaultDevice,
    #[cfg(feature = "backend_tch")]
    TchCpu,
    #[cfg(feature = "backend_tch")]
    TchCuda0,
    #[cfg(feature = "backend_tch")]
    TchVulkan,
}
