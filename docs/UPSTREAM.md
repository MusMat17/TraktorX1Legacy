# Upstream baseline

Primary implementation baseline:

- Opa-/x1-mk1-usb2midi
- upstream release reviewed: **1.3.3** (2026-06-22)
- license: Apache-2.0
- release 1.3.3 added macOS Intel x86_64 artifacts

Protocol research credited by upstream:

- joherold/traktor_x1

## Known upstream observations relevant to this fork

- Traktor Pro 4 is known to work with the bridge when the TSI and MIDI In/Out ports are configured.
- A historical SHIFT/mapping problem led to a revised TSI in upstream 1.3.1.
- An open 2026 report describes an Effect 3 LED occasionally clearing in Group FX mode under Traktor Pro 4. We keep this in the v0.1 test matrix.
- Upstream Intel release artifacts are produced on modern macOS runners. TraktorX1Legacy adds an explicit macOS 12 deployment target and validation because our first machine is Monterey.
