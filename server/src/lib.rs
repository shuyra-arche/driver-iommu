#![no_std]

//! IOMMU runtime server: owns the register window and retained pool, programs
//! the device directory table, Sv39 tables and queues, and answers the door with
//! acquire/publish/invalidate/quiesce/release for one coordinating Driver.
//!
//! Module bodies are imported verbatim from the frozen `driver/iommu` sources
//! with only module-path and dependency adapters applied.

extern crate alloc;

pub use iommu_api as api;
pub use iommu_declaration::bindings;
pub use iommu_hold as hold;

mod acquire;
mod binding;
mod cache_step;
mod context;
mod cookie;
mod core;
mod domain;
mod domain_id;
mod domain_range;
mod engine;
mod fence;
mod handshake;
mod invalidation;
mod layout;
mod order;
mod pages;
mod phase;
mod queue;
mod references;
mod regs;
mod settled;
mod table;
mod ticket;
pub mod serve;

/// Debug output adapter: the platform print lives in system-support.
pub use system_support::debug;

/// Ready mark adapter: the unit crate owns the mark constant.
pub use unit;

mod support;
