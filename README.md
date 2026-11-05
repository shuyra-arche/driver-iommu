# IOMMU driver

This provider owns the IOMMU declarations, public protocol, reference-holding
ledger and standalone `iommu` server image. The runtime comes from frozen
sqware `94e114614bec53424ac55e640fc80cf19a3e7107`.

The server owns its register window, device directory and Sv39 tables,
command/fault queues, completion fences and retained frame references. It
accepts only the Kernel-stamped coordinating Hub sender. PCI enumeration,
endpoint reset and VirtIO protocol policy remain with their owners.

## Layout

- `api/`: the existing six-operation public protocol and full Domain/Hub binding identities.
- `declaration/`: the existing `PROGRAM` and supplies; `BUILD` gives the new
  server image coordinates relative to this checkout.
- `hold/`: the existing single holds ledger and memory-effect interface.
- `server/`: `iommu-server` library and `iommu` binary. Register/queue/page-table
  effects and per-domain/binding state remain separate modules. Private supply
  and endpoint-lookup adapters keep their original authority checks.

The existing public API, `PROGRAM` and hold bodies are unchanged. No old
probe, guest scene, test suite or test-only cursor mutator is migrated here.
The server reuses the published `driver-shared::context` at immutable Driver
revision `bb3427e179e87b640e0d51115a44544fae231252` and the existing SDK pins.
It has no sqware or PCI runtime dependency, patch override or local Git backlink.

## Effect boundaries

`ESTABLISH` establishes/reuses a persistent Domain. `ACQUIRE` reserves a private
IOVA for the full Domain and Hub binding identity. `PUBLISH`, `INVALIDATE`,
`QUIESCE` and `RELEASE` remain distinct operations. Caller identity comes from
the Kernel; the runtime verifies the real Hub publisher before accepting work.

The hold ledger remains the reference authority. Publication, invalidation,
queue shutdown and readback retain their original ordering and failure/retry
behavior. Pending or failed isolation retains references; no host no-op fence
or replacement state account is introduced.

## Build and focused interface checks

Use the pinned `nightly-2026-09-04` toolchain:

```sh
cargo check --locked --offline -p iommu-server --target riscv64gc-unknown-none-elf
cargo build --locked --offline --release -p iommu-server --bin iommu --target riscv64gc-unknown-none-elf
cargo test --locked --offline -p iommu-api -p iommu-hold --target x86_64-unknown-linux-gnu
```

Prepare the immutable dependency cache before using `--offline`. The runtime
contains real RISC-V ordering instructions; host checks target the public API
and holds ledger rather than substituting hardware effects. The RV64 image
uses the linker script published by the existing execution package.

These are compilation and interface checks. No QEMU, SMP, real MMIO, DMA,
queue completion, hardware isolation or device-recovery acceptance is claimed.
Vanllia intake and deployment remain separate work.

## Source history and license

The unchanged `PROVENANCE.md` records the initial contract snapshot and its
existing tests. `RUNTIME-MIGRATION.md` and `runtime-source.json` record the
runtime source paths, hashes, authors, mechanical adaptations and limits.
The original source history and this repository's existing commits are kept;
this import does not rewrite either history.

The frozen source has no tracked license/notice file or manifest license grant.
This migration preserves existing headers and attribution, adds no license and
does not infer one from publication. No firmware binary is included.
