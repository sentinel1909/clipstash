use crate::data::DatabasePool;
use crate::ShortCode;
use crate::service::{self, ServiceError};
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

enum HitCounterMsg {
    Commit,
    Hit(ShortCode, u32)
}

pub struct HitCounter {
    tx: Sender<HitCounterMsg>
}

impl HitCounter {
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        todo!()
    }
}