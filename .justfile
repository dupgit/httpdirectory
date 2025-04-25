[private]
default:
  @just --list

name := "httpdirectory"

bump patch:
    cargo update
    cargo release version {{patch}} --no-confirm --execute
    cargo build --release
    cargo test --release
    cargo doc --no-deps
    cargo sbom | jq --sort-keys | jq '.files = (.files| sort_by(.SPDXID))' | jq '.packages = (.packages| sort_by(.SPDXID))' | jq '.relationships = (.relationships| sort_by(.spdxElementId))'>{{name}}.sbom.spdx.json
    git add Cargo.toml Cargo.lock {{name}}.sbom.spdx.json
    cargo release commit --no-confirm --execute
    cargo release tag --no-confirm --execute
