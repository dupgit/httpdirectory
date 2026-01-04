[private]
default:
    @just --list

name := "httpdirectory"

# Alias definitions for humans

alias t := test
alias d := document
alias c := coverage
alias ct := check-typos
alias cc := check-commits
alias p := publish
alias e := example
alias b := bench

# Installs all cargo tools to build a release or test coverage
install-dev-tools:
    cargo install cargo-release cargo-sbom cargo-tarpaulin cargo-nextest typos-cli conventional_commits_linter

# Linting commits from latest tag
check-commits:
    conventional_commits_linter --max-commit-title-length 75 $(git rev-list --tags --max-count=1)

# Bumps {patch} (major, minor or patch) version number and does a release
bump patch: check-typos check-commits
    # Verifying that the MSRV is still Ok.
    cargo msrv verify

    # Ensures that the source code is correctly formatted -> it should not modify anything
    cargo fmt

    # Checking that we do not have any untracked or uncommitted file
    git status -s | wc -l | grep '0'

    # Updating all dependencies
    cargo update

    # Bumping release version upon what has been asked on command line (major, minor or patch)
    cargo release version {{ patch }} --no-confirm --execute

    # Building, testing and building doc to ensure one can build with these dependencies
    cargo build --release
    cargo test --release --features test-output,test-helpers
    cargo doc --no-deps

    # Generetaing a Software Bills of Materials in SPDX∘format (sorting will reduce the diff size and allow one to figure out what has really changed)
    cargo sbom | jq --sort-keys | jq '.files = (.files| sort_by(.SPDXID))' | jq '.packages = (.packages| sort_by(.SPDXID))' | jq '.relationships = (.relationships| sort_by(.spdxElementId, .relatedSpdxElement))'>{{ name }}.sbom.spdx.json

    # Creating the release
    git add Cargo.toml Cargo.lock {{ name }}.sbom.spdx.json
    cargo release commit --no-confirm --execute
    cargo release tag --no-confirm --execute

# Runs tests for the project
test:
    cargo nextest run --features test-output,test-helpers
    cargo t --doc

# Creates the documentation and open it in a browser
document:
    cargo doc --no-deps --open

# Publishing in the git repository (with tags)
git-publish: check-commits
    git push
    git push --tags

# Publishing to crates.io
rust-publish:
    cargo publish

# Publishing to git and then to crates.io
publish: git-publish rust-publish

# Runs a coverage test and open it's result in a web browser
coverage:
    cargo tarpaulin --frozen --exclude-files benches/*.rs -o Html --features test-helpers
    open tarpaulin-report.html

# Runs benches (use module name in benches/ as bench_name)
bench bench_name='integration_bench':
    cargo bench --bench {{ bench_name }} --features test-helpers

# Check for typos
check-typos:
    typos src/ README.md tests examples .justfile benches ChangeLog

# Invoke clippy in pedantic mode
clippy:
    cargo clippy -- -W clippy::pedantic

# Run examples with hotpath feature
example example="debug_me":
    cargo r --release --example {{ example }} --features=hotpath
