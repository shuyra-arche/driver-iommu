# Runtime source checkpoint

This append imports the necessary running IOMMU driver from
`anreonyr/sqware@94e114614bec53424ac55e640fc80cf19a3e7107` into the existing
provider at `5d2038033efcbd55de77534b35e611729d4bd9bd`. The initial
API/declaration/hold checkpoint in `PROVENANCE.md` is retained unchanged.
`runtime-source.json` identifies each imported source blob, content hash,
last modifying commit and author. Full original history remains in sqware;
the old repository and its local checkout are not deleted or archived.

## Necessary adaptations

- The old `programs::driver::iommu` module becomes the server crate root.
  Its body modules keep their existing operations and state transitions.
- The new `iommu` binary calls the same `serve` function through the existing
  `mold::entry` and `execution::boot::Report` mechanism, preserving the success
  and failure reports. The execution package supplies the linker script.
- Existing local API/declaration/hold packages remain the only protocol,
  declaration and holds authorities. `declaration::BUILD` adds the owned
  server image coordinates; the `PROGRAM` body is unchanged.
- The published Driver Context at `bb3427e179e87b640e0d51115a44544fae231252`
  replaces the old shared Context. Private platform/retention/lookup adapters
  retain only the effects used by this server. The unused generic peer helper
  is omitted; no peer state or additional ledger is added.
- Core only used the old shared DMA module's reexport of
  `hub_api::window_mark`. It now aliases that same public API directly, so the
  unrelated Hub DMA client and device-coordinate structure are not copied.
- The old shared `fence iorw, iorw` body supplies ordering directly. It has no
  host no-op fallback. The old `#[cfg(test)]` IOVA cursor mutator is omitted.

## Preserved effects

Domain and binding identities stay full width. Sender checks, source marks and
permissions, IOVA bounds/non-reuse, retained frame acquisition/release,
publication-before-reachability accounting, command/fault queue handshakes,
invalidation fences, quiescence, isolation and teardown readbacks remain in the
same order. Failure keeps the existing pending references and retry state.
The server owns translation/reference effects; it does not reset PCI endpoints
or interpret the VirtIO protocol.

## Verification boundary

The relevant gates are a locked RV64 server check/build, an inspectable static
ELF, and the existing API/holds host checks. The lock extends the original
provider closure using immutable public SDK revisions; no sqware Git source,
legacy fixture, patch, replace or local Git source is required.

No old test suite or scene is copied. No QEMU/SMP, real register or DMA effects,
hardware completion, release approval or later Vanllia deployment is proved by
this source migration. Exact command logs, source comparisons, artifact hashes
and genuine DeepSeek namer/worker/reviewer evidence are retained in the task's
execution workspace.

## Attribution and license

The selected source history identifies `anreonyr` under the source-recorded
email identities. Existing source comments are retained. No tracked LICENSE,
COPYING or NOTICE file, or manifest license grant, exists at the frozen source
checkpoint. This append introduces no license and makes no additional license
or third-party permission claim. Registry/Git dependencies keep their own
upstream attribution and licensing; no firmware binary is copied.
