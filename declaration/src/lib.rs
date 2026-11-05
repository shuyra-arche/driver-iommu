#![no_std]
//! IOMMU-owned host-safe declarations shared by the driver runtime and composition.

pub use unit;

pub mod bindings;
mod image;
pub mod marks;
pub mod program;

pub use program::PROGRAM;
pub use image::BUILD;
