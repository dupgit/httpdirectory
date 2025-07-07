[private]
default:
  @just --list

name := "httpdirectory"

# Alias definitions
alias t := test
alias d := document
alias c := coverage
alias p := publish

# Installs all cargo tools to build a release or test coverage
install-dev-tools:
    cargo install cargo-release cargo-sbom cargo-tarpaulin cargo-nextest

# Bumps {patch} (major, minor or patch) version number and does a release
bump patch:
    # Ensures that the source code is correctly formatted -> it should not modify anything
    cargo fmt

    # Checking that we do not have any untracked or uncommitted file
    git status -s | wc -l | grep '0'

    # Updating all dependencies
    cargo update

    # Bumping release version upon what has been asked on command line (major, minor or patch)
    cargo release version {{patch}} --no-confirm --execute

    # Building, testing and building doc to ensure one can build with these dependencies
    cargo build --release
    cargo test --release
    cargo doc --no-deps

    # Generetaing a Software Bills of Materials in SPDX∘format (sorting will reduce the diff size and allow one to figure out what has really changed)
    cargo sbom | jq --sort-keys | jq '.files = (.files| sort_by(.SPDXID))' | jq '.packages = (.packages| sort_by(.SPDXID))' | jq '.relationships = (.relationships| sort_by(.spdxElementId, .relatedSpdxElement))'>{{name}}.sbom.spdx.json

    # Creating the release
    git add Cargo.toml Cargo.lock {{name}}.sbom.spdx.json
    cargo release commit --no-confirm --execute
    cargo release tag --no-confirm --execute

# Runs tests for the project
test:
    cargo nextest run
    cargo t --doc

# Creates the documentation and open it in a browser
document:
    cargo doc --no-deps --open

# Publishing in the git repository (with tags)
git-publish:
    git push
    git push --tags

# Publishing to crates.io
rust-publish:
    cargo publish

# Publishing to git and then to crates.io
publish: git-publish rust-publish

# Runs a coverage test and open it's result in a web browser
coverage:
    cargo tarpaulin --frozen --exclude-files benches/*.rs -o Html
    open tarpaulin-report.html

# Runs benches
bench:
    cargo bench
