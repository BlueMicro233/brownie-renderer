#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "macos")]
use std::process::Command;

pub struct CpuInfo {
    pub model: String,
    pub logical_cores: usize,
    pub has_avx: bool,
    pub has_avx2: bool,
}

impl CpuInfo {
    pub fn detect() -> Self {
        Self {
            model: detect_cpu_model(),
            logical_cores: std::thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(0),
            has_avx: detect_avx(),
            has_avx2: detect_avx2(),
        }
    }
}

fn detect_cpu_model() -> String {
    detect_cpu_model_impl()
        .filter(|model| !model.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

#[cfg(target_os = "macos")]
fn detect_cpu_model_impl() -> Option<String> {
    run_command("sysctl", &["-n", "machdep.cpu.brand_string"])
        .or_else(|| run_command("sysctl", &["-n", "hw.model"]))
        .or_else(|| match std::env::consts::ARCH {
            "aarch64" | "arm64" => Some("Apple Silicon".to_string()),
            arch => Some(format!("macOS ({arch})")),
        })
}

#[cfg(target_os = "linux")]
fn detect_cpu_model_impl() -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;
    cpuinfo.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == "model name").then(|| value.trim().to_string())
    })
}

#[cfg(target_os = "windows")]
fn detect_cpu_model_impl() -> Option<String> {
    std::env::var("PROCESSOR_IDENTIFIER").ok()
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn detect_cpu_model_impl() -> Option<String> {
    None
}

#[cfg(target_os = "macos")]
fn run_command(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8(output.stdout).ok()?;
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn detect_avx() -> bool {
    std::arch::is_x86_feature_detected!("avx")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn detect_avx() -> bool {
    false
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn detect_avx2() -> bool {
    std::arch::is_x86_feature_detected!("avx2")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn detect_avx2() -> bool {
    false
}
