# Versioning

## Overview
- Year-Semantic with Variant Suffixes Versioning
- Format: `YEAR[VARIANT].MAJOR.MINOR.PATCH`
  - YEAR: Release year (e.g., 2030).
  - VARIANT: Optional letter (a, b, c, ...) for multiple major releases in a year.
  - MAJOR: Significant feature changes or overhauls.
  - MINOR: Smaller features or improvements.
  - PATCH: Bug fixes or minor updates.
- Start using the next year’s identifier in June (flexible guideline).
- Examples:
  - `2030.2.3.7`: Second major release of 2030, minor version 3, patch 7.
  - `2030b.2.3.7`: Second variant of 2030’s releases, major version 2, minor 3, patch 7.

## `version` CLI
- Rust-based tool for managing Git tags in the versioning format.
- **Functionality**:
  - Prints the latest version tag (e.g., `2030b.2.3.7`).
  - Copies the year and variant prefix (e.g., `2030b.`) to the clipboard for quick use.
- **Usage**: Run `version` in a Git repository.

## Future Considerations
- Custom `git` integration with versioning support.
