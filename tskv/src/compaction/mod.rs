mod compact;
mod flush;
mod picker;

pub use compact::*;
pub use flush::*;
pub use picker::*;

use crate::{
    memcache::MemCache,
    summary::VersionEdit,
    tseries_family::{ColumnFile, Version},
};

#[async_trait::async_trait]
pub trait CompactionEngine: Clone + Sync + Send {
    async fn apply(&mut self, edits: Vec<VersionEdit>) -> crate::error::Result<()>;
}
pub struct CompactReq {
    files: (u32, Vec<std::sync::Arc<ColumnFile>>),
    version: std::sync::Arc<Version>,
    cf: u32,
    out_lvl: u32,
}

pub struct FlushReq {
    pub mems: Vec<(u32, std::sync::Arc<MemCache>)>,
    pub wait_req: u64,
}

impl FlushReq {
    pub fn new(mems: Vec<(u32, std::sync::Arc<MemCache>)>, wait_req: u64) -> Self {
        Self { mems, wait_req }
    }
}
