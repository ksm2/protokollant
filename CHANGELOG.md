# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Use setup-protokollant-action

## [0.4.1] - 2023-03-12

### Changed

- Use platform-agnostic build in Release

## [0.4.0] - 2023-03-12

### Added

- Support for macOS Silicon

## [0.3.0] - 2023-03-11

### Added

- Support more platforms: windows and macOS Intel
- Add flag to add unreleased section
- Start next iteration using `next-iteration`
- Skip modifying changelog using `--no-changelog`

### Fixed

- Bump `Cargo.lock` on release
- Add help for `--diff`

## [0.2.0] - 2023-03-09

### Added

- Parse changelog files
- Generate changelog files
- Version bump using `major`, `minor` or `patch`
- Option to display `--diff`
- Bump `Cargo.toml` on release

## [0.1.0] - 2023-03-05

### Added

- Created repository

[unreleased]: https://github.com/ksm2/protokollant/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/ksm2/protokollant/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/ksm2/protokollant/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ksm2/protokollant/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ksm2/protokollant/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ksm2/protokollant/releases/tag/v0.1.0
