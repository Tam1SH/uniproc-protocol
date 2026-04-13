use ogurpchik::codecs::rkyv_protocol::RkyvCodec;
use rkyv::{Archive, Deserialize, Serialize};

pub mod services {
    pub const LINUX_AGENT: &str = "WSL";
    pub const WINDOWS_AGENT: &str = "HOST";
}

// ───────────────────────────── Linux ──────────────────────────────────

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Default)]
#[rkyv(derive(Debug))]
pub struct LinuxMachineStats {
    pub total_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub cached_kb: u64,

    pub busy_ns: u64,
    pub last_tsc: u64,

    pub vsock_rx_bytes: u64,
    pub vsock_tx_bytes: u64,
    pub p9_rx_bytes: u64,
    pub p9_tx_bytes: u64,

    pub tcp_tx_lo_bytes: u64,
    pub tcp_rx_lo_bytes: u64,
    pub tcp_tx_remote_bytes: u64,
    pub tcp_rx_remote_bytes: u64,
    pub udp_tx_lo_bytes: u64,
    pub udp_rx_lo_bytes: u64,
    pub udp_tx_remote_bytes: u64,
    pub udp_rx_remote_bytes: u64,
    pub uds_tx_bytes: u64,
    pub uds_rx_bytes: u64,

    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_read_iops: u64,
    pub disk_write_iops: u64,

    pub pipe_read_bytes: u64,
    pub pipe_write_bytes: u64,
    pub sendfile_bytes: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct LinuxProcessStats {
    pub global_pid: u32,
    pub local_pid: u32,
    pub mnt_ns: u64,
    pub pid_ns: u64,
    pub name: [u8; 64],

    pub cpu_percent: f32,
    pub rss_kb: u64,
    pub last_active_ns: u64,

    pub vsock_rx_bytes: u64,
    pub vsock_tx_bytes: u64,
    pub p9_rx_bytes: u64,
    pub p9_tx_bytes: u64,

    pub tcp_tx_lo_bytes: u64,
    pub tcp_rx_lo_bytes: u64,
    pub tcp_tx_remote_bytes: u64,
    pub tcp_rx_remote_bytes: u64,
    pub udp_tx_lo_bytes: u64,
    pub udp_rx_lo_bytes: u64,
    pub udp_tx_remote_bytes: u64,
    pub udp_rx_remote_bytes: u64,
    pub uds_tx_bytes: u64,
    pub uds_rx_bytes: u64,

    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_read_iops: u64,
    pub disk_write_iops: u64,

    pub pipe_read_bytes: u64,
    pub pipe_write_bytes: u64,
    pub sendfile_bytes: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum LinuxEnvironmentKind {
    Unknown,
    CurrentDistro { name: String },
    DockerContainer { id: String },
    UnknownExternalNamespace,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct LinuxEnvironmentInfo {
    pub mnt_ns: u64,
    pub pid_ns: u64,
    pub kind: LinuxEnvironmentKind,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct LinuxDockerContainerInfo {
    pub id: String,
    pub mnt_ns: u64,
    pub pid_ns: u64,
    pub api_version: String,
    pub raw_json: String,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct LinuxReport {
    pub machine: LinuxMachineStats,
    pub processes: Vec<LinuxProcessStats>,
    pub environments: Vec<LinuxEnvironmentInfo>,
    pub docker_containers: Vec<LinuxDockerContainerInfo>,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum LinuxRequest {
    GetReport,
    Ping,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum LinuxResponse {
    Report(LinuxReport),
    Pong,
}

pub type LinuxCodec = RkyvCodec<LinuxRequest, LinuxResponse>;
pub type MachineStats = LinuxMachineStats;
pub type ProcessStats = LinuxProcessStats;

// ───────────────────────────── Windows ────────────────────────────────

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Default)]
#[rkyv(derive(Debug))]
pub struct WindowsAgentConfig {
    pub memory_interval_ms: u64,
    pub cpu_interval_ms: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum ProcessPriority {
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum ProcessCommand {
    Kill { pid: u32 },
    Suspend { pid: u32 },
    Resume { pid: u32 },
    SetPriority { pid: u32, priority: ProcessPriority },
    SetAffinity { pid: u32, mask: u64 },
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum ServiceCommand {
    Start { name: String },
    Stop { name: String },
    Pause { name: String },
    Resume { name: String },
    Restart { name: String },
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum CommandResult {
    Ok,
    Err(u32),
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Default)]
#[rkyv(derive(Debug))]
pub struct WindowsMachineStats {
    pub total_physical_kb: u64,
    pub available_physical_kb: u64,
    pub used_physical_kb: u64,
    pub cpu_percent: f32,
    pub cpu_max_mhz: u64,
    pub cpu_current_mhz: u64,

    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_read_iops: u64,
    pub disk_write_iops: u64,

    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct WindowsProcessStats {
    pub pid: u32,
    pub parent_pid: u32,
    pub session_id: u32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub package_full_name: String,
    pub package_relative_app_id: String,
    pub cpu_percent: f32,
    pub working_set_kb: u64,
    pub private_bytes_kb: u64,
    pub peak_working_set_kb: u64,

    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_read_iops: u64,
    pub disk_write_iops: u64,

    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct WindowsReport {
    pub machine: WindowsMachineStats,
    pub processes: Vec<WindowsProcessStats>,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum WindowsRequest {
    GetReport,
    Ping,
    SetConfig(WindowsAgentConfig),
    ProcessCommand(ProcessCommand),
    ServiceCommand(ServiceCommand),
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub enum WindowsResponse {
    Report(WindowsReport),
    Pong,
    ConfigApplied,
    CommandResult(CommandResult),
}

pub type WindowsCodec = RkyvCodec<WindowsRequest, WindowsResponse>;
