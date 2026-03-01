use rkyv::{Archive, Deserialize, Serialize};
use ogurpchik::codecs::rkyv_protocol::RkyvCodec;

pub mod services {
    pub const GUEST: &'static str = "WSL";
    pub const HOST: &'static str = "HOST";
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum AgentRequest {
    ReportStats(Vec<StatEntry>),
    Ping,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum AgentResponse { Ok, Pong }

pub type AgentCodec = RkyvCodec<AgentRequest, AgentResponse>;

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum HostRequest {
    SetConfig(Config),
    Ping,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub enum HostResponse { Ok, Pong }

pub type HostCodec = RkyvCodec<HostRequest, HostResponse>;

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct StatEntry {
    pub pid: u32,
    pub cpu_ns: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
#[rkyv(derive(Debug))]
pub struct Config {
    pub sample_interval_ms: u64,
}