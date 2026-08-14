# Reusable GitHub Actions

This repository now uses shared actions from
[legra-ai/github-actions](https://github.com/legra-ai/github-actions).

## Migration Notice

All local actions have been migrated to use the shared actions
repository. Workflows now reference:

```yaml
uses: legra-ai/github-actions/.github/actions/action-name@v0.0.1
```

## Available Shared Actions

See the
[shared actions repository](https://github.com/legra-ai/github-actions)
for complete documentation of all available actions.

### Actions Used in This Repository

- `setup-cocogitto` - Install Cocogitto for version management
- `generate-changelog` - Generate changelog from conventional commits
- `setup-cargo-fmt-toml` - Install cargo-fmt-toml for Cargo.toml
  formatting
- `setup-cargo-version-info` - Install cargo-version-info for version
  management

## Usage

```yaml
- name: Setup Cocogitto
  uses: legra-ai/github-actions/.github/actions/setup-cocogitto@v0.0.1

- name: Generate changelog
  uses: legra-ai/github-actions/.github/actions/generate-changelog@v0.0.1
  with:
    release-tag: v0.1.0
```

## Versioning

All shared actions support versioning via inputs and environment
variables. See the
[shared actions documentation](https://github.com/legra-ai/github-actions)
for details.
