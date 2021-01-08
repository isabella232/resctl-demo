// Copyright (c) Facebook, Inc. and its affiliates.

// The individual bench implementations under bench/ inherits all uses from
// this file. Make common stuff available.
use anyhow::{bail, Result};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Write;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

use super::job::Job;
use super::progress::BenchProgress;
use super::run::RunCtx;
use super::study::*;
use rd_agent_intf::SysReq;
use resctl_bench_intf::JobSpec;

use util::*;

lazy_static::lazy_static! {
    pub static ref HASHD_SYSREQS: HashSet<SysReq> = FromIterator::from_iter(
        vec![
                SysReq::AnonBalance,
                SysReq::SwapOnScratch,
                SysReq::Swap,
                SysReq::HostCriticalServices,
        ]
    );
}

lazy_static::lazy_static! {
    pub static ref BENCHS: Arc<Mutex<Vec<Box<dyn Bench>>>> = Arc::new(Mutex::new(vec![]));
}

pub struct BenchDesc {
    pub kind: String,
    pub takes_propsets: bool,
}

impl BenchDesc {
    pub fn new(kind: &str) -> Self {
        Self {
            kind: kind.into(),
            takes_propsets: false,
        }
    }

    pub fn takes_propsets(mut self) -> Self {
        self.takes_propsets = true;
        self
    }
}

pub trait Bench: Send + Sync {
    fn desc(&self) -> BenchDesc;
    fn parse(&self, spec: &JobSpec) -> Result<Box<dyn Job>>;
}

fn register_bench(bench: Box<dyn Bench>) -> () {
    BENCHS.lock().unwrap().push(bench);
}

mod hashd_params;
mod iocost_params;
mod iocost_qos;
mod storage;

pub fn init_benchs() -> () {
    register_bench(Box::new(storage::StorageBench {}));
    register_bench(Box::new(iocost_params::IoCostParamsBench {}));
    register_bench(Box::new(hashd_params::HashdParamsBench {}));
    register_bench(Box::new(iocost_qos::IoCostQoSBench {}));
}