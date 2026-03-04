use rkyv::{Archive, Deserialize, Serialize};
use ogurpchik::codecs::rkyv_protocol::RkyvCodec;

pub mod services {
    pub const GUEST: &'static str = "WSL";
    pub const HOST:  &'static str = "HOST";
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct ProcessStats {
    pub global_pid: u32,
    pub local_pid:  u32,
    pub name:       [u8; 64],

    pub cpu_runtime_ns: u64,
    pub rss_kb:         u64,
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

    pub disk_read_bytes:  u64,
    pub disk_write_bytes: u64,

    pub disk_read_iops:  u64,
    pub disk_write_iops: u64,

    pub pipe_read_bytes:  u64,
    pub pipe_write_bytes: u64,

    pub sendfile_bytes: u64,
}

impl ProcessStats {
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        std::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Default)]
#[rkyv(derive(Debug))]
pub struct MachineStats {
    pub busy_ns:  u64,
    pub last_tsc: u64,

    pub total_kb:     u64,
    pub free_kb:      u64,
    pub cached_kb:    u64,
    pub available_kb: u64,
    pub used_kb:      u64,

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

    pub disk_read_bytes:  u64,
    pub disk_write_bytes: u64,

    pub disk_read_iops:  u64,
    pub disk_write_iops: u64,

    pub pipe_read_bytes:  u64,
    pub pipe_write_bytes: u64,

    pub sendfile_bytes: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct AgentReport {
    pub machine:   MachineStats,
    pub processes: Vec<ProcessStats>,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum HostRequest {
    GetReport,
    Ping
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum HostResponse {
    Report(AgentReport),
    Pong
}

pub type HostCodec = RkyvCodec<HostRequest, HostResponse>;

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum AgentRequest {
    Ping,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum AgentResponse {
    Pong,
}

pub type AgentCodec = RkyvCodec<AgentRequest, AgentResponse>;