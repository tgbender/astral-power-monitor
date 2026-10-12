# Attribution and upstream notices

Astral Power Monitor is maintained independently. It is not an official ASUS,
NVIDIA, or upstream-project product.

## Original Rust sensor monitor

- Project: [Timic3/astral-power-monitoring](https://github.com/Timic3/astral-power-monitoring)
- Starting revision: `6ae55e6bce2af9c1572f260ee99f07bca3b6a351`
- License: Unlicense, preserved in [LICENSE](LICENSE)

The original project established reading the IT8915 sensor through NVIDIA NVAPI
and interpreting the six reversed current/voltage blocks. That approach remains
the foundation of `src/monitor.rs` and `src/model.rs`, although their implementation
has been refactored and extended.

## Python protection guard

- Project: [humza-khalid/12vhpwr-guard](https://github.com/humza-khalid/12vhpwr-guard)
- License: MIT, preserved verbatim in [LICENSE-12VHPWR-GUARD](LICENSE-12VHPWR-GUARD)

The tier ladder and protective behavior in `src/protection.rs`, and portions of
the GPU mitigation/recovery approach in `src/actions.rs`, were adapted from this
project. This is a Rust reimplementation; the Python application is not bundled.

## New work and dependencies

The new application code and generated icon/audio assets are supplied under the
Unlicense. These notices do not replace the licenses of Rust dependencies such
as `nvapi-sys`, `windows-sys`, `rusqlite`, or `serde_json`, or the bundled SQLite
library. Dependency versions are recorded in `Cargo.lock`.
