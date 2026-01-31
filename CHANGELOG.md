# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-31

### Added
- Initial release of Gambas Matcher Mock
- Core proxy server functionality with Tokio and Hyper
- Request matching based on:
  - HTTP method
  - Exact path matching
  - Regex path matching
  - Header matching
  - Query parameter matching
- Mock response generation with:
  - Custom status codes
  - Custom headers
  - Inline body content
  - File-based body content
- YAML-based configuration system
- CLI with command-line arguments
- Comprehensive logging with configurable verbosity
- Example configuration files
- Example mock data files
- Automated test script with validation
- Complete documentation in README
- Unit tests for core functionality
- MIT License

### Features
- Transparent API interception
- Smart request routing
- Flexible mock rules
- File-based response support
- Easy configuration
- High performance async operation

[0.1.0]: https://github.com/lionelkouame/gambas-matcher-mock/releases/tag/v0.1.0
