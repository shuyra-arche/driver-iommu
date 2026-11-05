#![no_std]
#![no_main]

//! IOMMU server boot image. The body lives in `iommu_server::serve::serve`.

extern crate iommu_server;

#[mold::entry]
fn main() -> execution::boot::Report<'static> {
    match iommu_server::serve::serve() {
        Ok(()) => execution::boot::Report::note(env::EXIT_OK, "iommu: done"),
        Err(why) => execution::boot::Report::note(env::EXIT_FAULT, why),
    }
}
