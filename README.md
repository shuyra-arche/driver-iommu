# IOMMU driver

This provider owns the IOMMU declarations, public protocol, reference-holding
ledger and standalone `iommu` server image. The runtime comes from frozen
sqware `94e114614bec53424ac55e640fc80cf19a3e7107`.

The server owns its register window, device directory and Sv39 tables,
command/fault queues, completion fences and retained frame references. It
accepts only the Kernel-stamped device-coordination publisher. PCI enumeration,
endpoint reset and VirtIO protocol policy remain with their owners.

## Layout

- `api/`: the existing six-operation public protocol and full Domain/Hub binding identities.
- `declaration/`: the existing `PROGRAM` and supplies; `BUILD` gives the new
  server image coordinates relative to this checkout.
- `hold/`: the existing single holds ledger and memory-effect interface.
- `server/`: `iommu-server` library and `iommu` binary. Register/queue/page-table
  effects and per-domain/binding state remain separate modules. Private supply
  and endpoint-lookup adapters keep their original authority checks.
- `unit.toml`: schema 1 runtime policy consumed by Vanllia's Unit discovery.

The existing public API, `PROGRAM` and hold bodies are unchanged. No old
probe, guest scene, test suite or test-only cursor mutator is migrated here.
The server reuses the published `driver-shared::context`. The workspace root
selects its SDK dependencies from immutable public Git revisions, including
legacy transitive edges; see [canonical API validation](CANONICAL-API-VALIDATION.md).
It has no sqware or PCI runtime dependency or local Git backlink.

## Runtime declaration

Root `unit.toml` preserves the compiled `PROGRAM` policy: supervisor image
`iommu`, shared `service/iommu` subtree, operation bits 279, the existing entry
source mark, READY_MARK signal, resident ending, and Hub retirement at namespace
0, face `retire`. System completes Operator and Identity bootstrap before it
creates startup Units; the actual Unit dependency is `hub` in both `requires`
and `after`. No new cluster or platform policy is introduced.

Both supplies request FETCH access to existing System collection manifests:
`memory/retention` at the retention slot and `platform/iommu` at the region slot.
Their decimal marks are quoted to preserve all 64 bits. The runtime still
verifies the System sender and manifest shape, claims the FETCH | VEST retention
authority, and accepts only a FETCH | STORE | ONLY register window. The document
does not widen either supplied authority.

Production SDK revisions and `PROGRAM` remain the preservation reference.
The focused `tests/unit-document` workspace pins Vanllia's Unit SDK
`ad50b803c3bd5cd2ef90010d80badf23551e88d5` and verifies policy preservation,
binary record/runtime conversion, supplies, retirement and existing image
coordinates. It adds no legacy test scene.

## Effect boundaries

`ESTABLISH` establishes/reuses a persistent Domain. `ACQUIRE` reserves a private
IOVA for the full Domain and Hub binding identity. `PUBLISH`, `INVALIDATE`,
`QUIESCE` and `RELEASE` remain distinct operations. Caller identity comes from
the Kernel; the runtime verifies the declared `device/coord` publisher before
accepting work. The coordination protocol and full binding identity belong to
Hub, while the selected VirtIO block coordinator publishes that endpoint.

PCI inspection and placement share a PCI task; the IOMMU engine is a separate
authenticated peer at `iommu/entry`. IOMMU takes its region and retention from
System, while BAR and DMA source handoff remain dynamic at PCI/coordinator and
execution respectively. No static BAR or DMA source binding is declared here.

The hold ledger remains the reference authority. Publication, invalidation,
queue shutdown and readback retain their original ordering and failure/retry
behavior. Pending or failed isolation retains references; no host no-op fence
or replacement state account is introduced.

`iommu-hold` uses only `AcquireSource`, `FlatReference`, `PieToken`, and
`TaskId` from `kernel-api-env`, imported as `env` from Kernel revision
`a9a1d8d788213597adb69605d1787bdef65d2b64`. This direct API dependency keeps
Kernel Client out of the ledger's normal/build closure when Kernel Server
uses it. That package-level pin stays fixed. The standalone workspace's root
composition selects API revision `90d43dbf3ee43fd146f487718eb39e88f2507c05`
for the whole driver, while consumers such as Kernel own their own composition.

## Build and focused interface checks

The repository pins `nightly-2026-09-04`; this migration was verified with the
installed `nightly-2026-10-09` toolchain. Standalone builds remain supported:

```sh
cargo check --locked --offline -p iommu-server --target riscv64gc-unknown-none-elf
cargo build --locked --offline --release -p iommu-server --bin iommu --target riscv64gc-unknown-none-elf
cargo test --locked --offline -p iommu-api -p iommu-hold --target x86_64-unknown-linux-gnu
```

These standalone commands pass using only the root's public Git selections and
committed lock: RV64 Server check, release `iommu` ELF, and 18 API/hold host
tests. The earlier `90f59ef` checkpoint introduced six dual-ABI type errors;
the dependency selections in this revision repair that regression without
changing Server or Runtime bodies. The effective Server/API/hold graphs share
one canonical API owner; the ledger's normal/build graph has no Client.
The validation record documents the exact source selections, Cargo's deprecated
`[replace]` tradeoff, CLI overlay checks, and remaining acceptance limits.

Prepare the immutable dependency cache before using `--offline`. The runtime
contains real RISC-V ordering instructions; host checks target the public API
and holds ledger rather than substituting hardware effects. The RV64 image
uses the linker script published by the existing execution package.

The runtime declaration checks also pass with Vanllia's
`nightly-2026-10-09` toolchain:

```sh
cargo test --locked --offline --manifest-path tests/unit-document/Cargo.toml \
  --target x86_64-unknown-linux-gnu
```

These are compilation and interface checks. No QEMU, SMP, real MMIO, DMA,
queue completion, hardware isolation or device-recovery acceptance is claimed.
Vanllia composition acceptance is reported separately from these provider checks.

## Source history and license

The unchanged `PROVENANCE.md` records the initial contract snapshot and its
existing tests. `RUNTIME-MIGRATION.md` and `runtime-source.json` record the
runtime source paths, hashes, authors, mechanical adaptations and limits.
The original source history and this repository's existing commits are kept;
this import does not rewrite either history.

The frozen source has no tracked license/notice file or manifest license grant.
This migration preserves existing headers and attribution, adds no license and
does not infer one from publication. No firmware binary is included.
