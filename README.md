# IOMMU Unit

IOMMU owns translation and retained-reference effects for Shuyra: register access,
Sv39 tables, command/fault queues, completion fences and per-binding state.
PCI endpoint reset and VirtIO protocol policy remain with their providers.

## Layout and safety boundaries

- `api/`: Domain and Hub binding identities and the six-operation protocol.
- `hold/`: reference authority and memory-effect interface.
- `server/`: private runtime implementation and the `iommu` binary.
- `declaration/`: compatibility Rust policy/build declaration.
- `unit.toml`: startup after Hub; System retention and IOMMU-region supplies.
- `tests/unit-document/`: independent host policy/record checks.

ESTABLISH, ACQUIRE, PUBLISH, INVALIDATE, QUIESCE and RELEASE have distinct effects.
The runtime authenticates the Kernel-stamped `device/coord` publisher.
Pending or failed isolation retains references and retry state. Publication,
invalidation, queue shutdown and readbacks must preserve their ordering;
a host no-op fence cannot replace device completion.

The hold ledger imports Kernel API directly so its normal/build graph does not
pull in Kernel Client. Root `Cargo.toml` uses explicit `[replace]` selections
to keep the standalone driver graph on one API identity; consumers own their
combined source selection.

## Build and test

Install Rust through rustup; the checkout pins nightly-2026-09-04 and RV64.
Uncached dependencies require network access.

```sh
cargo check --locked -p iommu-server --target riscv64gc-unknown-none-elf
cargo build --locked --release -p iommu-server --bin iommu --target riscv64gc-unknown-none-elf
cargo test --locked -p iommu-api -p iommu-hold --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
```

Host checks cover API, ledger and declaration behavior. MMIO, DMA isolation,
queue completion, recovery and SMP require runtime validation through
[Vanllia](https://github.com/shuyra-arche/vanllia).

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
