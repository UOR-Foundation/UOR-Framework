# Releasing

## Prerequisites

- `CARGO_REGISTRY_TOKEN` org secret configured at `github.com/UOR-Foundation`
  (Settings > Secrets and variables > Actions)

## Release Process

1. Update `version` in the workspace root `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "X.Y.Z"
   ```

2. Commit the version bump:
   ```sh
   git add Cargo.toml Cargo.lock
   git commit -m "Bump version to X.Y.Z"
   ```

3. Tag and push:
   ```sh
   git tag vX.Y.Z
   git push origin main --tags
   ```

4. The release workflow will automatically:
   - Validate the tag matches the Cargo.toml version
   - Run all checks (fmt, clippy, test, conformance)
   - Verify crate packaging with `cargo publish --dry-run`
   - Create a GitHub Release with ontology artifacts
   - Publish `uor-foundation` to crates.io

## Troubleshooting

- **Tag/version mismatch**: The workflow fails early if the tag version
  does not match `Cargo.toml`. Fix the version and re-tag.
- **crates.io publish failure**: The GitHub Release will already exist.
  Fix the issue, delete and re-create the tag, or manually run
  `cargo publish -p uor-foundation`.
- **Version already published**: crates.io does not allow re-publishing
  the same version. Bump the version and create a new tag.
