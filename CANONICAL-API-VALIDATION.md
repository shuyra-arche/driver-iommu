# Canonical API dependency validation

Validated on 2026-11-05 with `nightly-2026-10-09`. The repository's
`nightly-2026-09-04` toolchain pin is unchanged and was not revalidated here.
Server operations, Runtime bodies, linker metadata, Unit policy, and test
assertions are unchanged from default-branch baseline
`d3d3a4ccf796123eb47d72f5c2751c3756f2b0af`.

## Regression and correction

Using the same Oct 9 compiler, Cargo cache and raw commands, baseline `d3d3a4c`
passes Server RV64 check and release build. This PR's `90f59ef` checkpoint
introduced six type-identity errors: the ledger used canonical API types while
`references.rs` and `core.rs` still received independently defined legacy
`AcquireSource`, `FlatReference`, `PieToken`, and `TaskId` types. The Server
bodies were identical at both revisions. That failure was introduced by this
migration; it was not a pre-existing failure or a withdrawal of standalone
support.

This revision corrects the dependency composition. `hold/Cargo.toml` keeps its
immutable API anchor `a9a1d8d788213597adb69605d1787bdef65d2b64`; the standalone
root selects one common API revision without requiring Kernel to follow a new
IOMMU pin. Root selections do not propagate when Kernel consumes `iommu-hold`.

## Public source selections

`Cargo.toml` contains seven exact Package ID replacements, all version `0.1.0`.
The full original repository URL, revision, package name and version are part
of each key; these are root composition decisions, not wildcard URL aliases.

| Package | Original revision | Selected public revision |
| --- | --- | --- |
| `env` | `d3000253` | [`f54f9a1ab3f0df3631fb80ff2bf5d0cfaecbe927`](https://github.com/shuyra-arche/env/commit/f54f9a1ab3f0df3631fb80ff2bf5d0cfaecbe927) |
| `wire` | `0ff77dc4` | [`4b7619fda4bbd202e9a5c952872a84c2145c681e`](https://github.com/shuyra-arche/wire/commit/4b7619fda4bbd202e9a5c952872a84c2145c681e) |
| `mold` | `f52501f4` | [`1addd302789e6a1c0dea6a7adf27e975a05d5ada`](https://github.com/shuyra-arche/mold/commit/1addd302789e6a1c0dea6a7adf27e975a05d5ada) |
| `kernel-api-env` | `a9a1d8d7` | [`90d43dbf3ee43fd146f487718eb39e88f2507c05`](https://github.com/shuyra-arche/kernel/commit/90d43dbf3ee43fd146f487718eb39e88f2507c05) |
| `system-client` | `5a792ff2` | [`8eea7c9039cac0e6ada4a59c612dc6c98ef9bc45`](https://github.com/shuyra-arche/system/commit/8eea7c9039cac0e6ada4a59c612dc6c98ef9bc45) |
| `system-support` | `5a792ff2` | [`8eea7c9039cac0e6ada4a59c612dc6c98ef9bc45`](https://github.com/shuyra-arche/system/commit/8eea7c9039cac0e6ada4a59c612dc6c98ef9bc45) |
| `driver-shared` | `bb3427e1` | [`10379c3a1a47f26c82dfbd66438a73baed5016d6`](https://github.com/shuyra-arche/driver/commit/10379c3a1a47f26c82dfbd66438a73baed5016d6) |

The Server directly selects published execution revision
`ea49192e5df8bf744e083ced23ff0908b5bab3a3`. The selected System and Driver
packages also use that execution revision. Their reviewed dependency changes
are [System Draft #4](https://github.com/shuyra-arche/system/pull/4) and
[Driver Draft #8](https://github.com/shuyra-arche/driver/pull/8).

Same-repository `[patch]` to another Git revision was tested and rejected by
Cargo as pointing to the same source. A direct `execution` replacement was
also tested with an empty-lock one-dependency fixture: Cargo rejects two
`links = "execution"` packages before applying that replacement. The selected
System/Driver parent packages have no `links` identity and remove all old
execution edges. `DEP_EXECUTION_LINKER_SCRIPT` and the Server build script
remain unchanged.

Cargo [deprecates `[replace]`](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-replace-section).
Its use here is a bounded compatibility choice that preserves public Git
sources and existing standalone commands. A new dependency revision is not
covered by an old exact key: `--locked` refuses an unresolved lock update, and
any deliberate dependency update must repeat the compilation and active-owner
checks below. The root's selections and lock must be maintained together.
Library consumers own their root selection; no path patch or second ABI is
published. Lock entries with `replace = ...` retain provenance and are not
additional active compiled owners.

Remove the corresponding exact selections once all formerly replaced manifest
pins have publicly converged on the same canonical API/Client revisions and
clean raw check/release, host tests, owner checks, CLI overlay, and Kernel
consumer checks pass without them. Regenerate the lock to remove the obsolete
replacement provenance; changing URL spellings or copying ABI definitions is
not a replacement for that dependency convergence.

## Raw standalone results

The following commands used the installed Oct 9 `cargo`, matching `RUSTC` and
`RUSTDOC`, and `CARGO_HOME=/workspace/.sqware-tools/cargo`. Public Git sources
were fetched before running offline; no Cargo `--config` overlay was supplied.

```sh
cargo check --locked --offline -p iommu-server --target riscv64gc-unknown-none-elf
cargo build --locked --offline --release -p iommu-server --bin iommu --target riscv64gc-unknown-none-elf
cargo test --locked --offline -p iommu-api -p iommu-hold --target x86_64-unknown-linux-gnu
cargo test --locked --offline --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
cargo metadata --locked --offline --format-version 1
cargo tree --locked --offline -p iommu-hold --target all --edges normal,build
```

Worker outputs used `/tmp/iommu-standalone-fixed-{server,host,unit}` as separate
`--target-dir` values. The independent reviewer repeated the checks separately.

- Server check and release build: exit 0, with the same 19 existing dead-code
  warnings seen on the baseline.
- API/hold host tests: 18 passed, none ignored; Unit document tests: 3 passed.
- Effective normal/build graphs: Server 37 packages, protocol API 24, hold 10.
  Each contains exactly one `kernel-api-env`, `kernel-api-wire`, and
  `kernel-api-macros`, all at `90d43dbf`. Server has one Client and one execution
  provider; hold has no Client, execution, legacy `env`, `wire`, or `mold`.
- Public `Cargo.lock` SHA-256:
  `71a42f79ef4b41f4cca229881718438503bc856a0207b22c7edbec8300aec57f`.
- Raw release `iommu`: statically linked RISC-V ELF64, 354208 bytes, SHA-256
  `1f2118e11390b61b04c88f4775e6354c11849a0f94f2d5bf79b14e299f670da9`.

The Unit-document workspace retains its separate legacy SDK preservation
reference; it is not part of the Server's active dependency graph.

## CLI overlay coexistence

A `/tmp` mirror of this candidate was resolved, checked and release-built
with Vanllia's previously generated native Cargo overlay. Its config SHA-256
was `8863f7f91f6bda1e9856f4110362792495b3f5cbf9b2f5a18ce8473dbb4ecc69`.
The original config was supplied with `--config`, followed by a second
`--config 'resolver.lockfile-path="/tmp/iommu-standalone-overlay-candidate/overlay-lock/Cargo.lock"'`.
Metadata resolved that separate lock before the locked check and release build.
Neither the public source lock nor the original receipt config/lock changed.

Both builds passed. Metadata still selected one API provider, and hold remained
Client-free. Cargo reported seven unused exact replacements because the CLI
patches selected those packages first; the broad overlay also reported unused
patches. These warnings were retained. There was no replace/patch conflict.

The coexistence-probe ELF is a separate 354296-byte artifact, SHA-256
`2e7c4a41fc46292c7fb8090a1c12c9c15d8211a00320e44e79b68e7ab144db32`;
it is not the raw ELF above or a fresh product receipt. Fresh product selection
and complete image acceptance are performed after publishing this revision.
The worker command logs, metadata, owner summaries and failed-fixture evidence
are retained in `/tmp/iommu-standalone-evidence/` for this review.

No QEMU, SMP, real MMIO, DMA, queue-completion, isolation or device-recovery
acceptance is claimed by these compilation and interface checks.
