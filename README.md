# Versioning

## Overview
- **Year-Phase Versioning with Variant and Stability Suffixes**.
- Format: `YEAR[VARIANT].PHASE.MAJOR.MINOR.PATCH[-b|-p]`
  - **YEAR**: Release year (e.g., 2026), omitted for internal phase, starts in June (flexible guideline).
  - **VARIANT**: Optional letter (a, b, c, ...) for distinct release streams in a year (e.g., redesigns, parallel projects).
  - **PHASE**: `i` (internal), `b` (beta), `p` (public beta), `s` (stable).
  - **MAJOR**: Breaking changes since last release (0 means none). Resets MINOR and PATCH to 0.
  - **MINOR**: New features or improvements. Resets PATCH to 0.
  - **PATCH**: Bug fixes or minor updates.
  - **Suffix**: Optional `-b` (beta) or `-p` (public beta) for non-stable releases in stable or public beta phases.
- **Phase Rules**:
  - Internal phase: Omits `YEAR`, uses `PHASE=i`, no `-b` or `-p` (e.g., `i.0.32.102`).
  - Other phases: Include `YEAR`, use `PHASE=b`, `p`, or `s`. Beta phase omits `-b`/`-p`. Public beta uses `-b` (not `-p`). Stable uses `-b` or `-p`.
  - Phases: Internal (`i`), beta (`b`), public beta (`p`), stable (`s`). Can skip beta or public beta, but beta testing is recommended.
  - Version resets to `0.0.0` when entering a new phase or variant (e.g., `2026.s.0.0.0` to `2026a.s.0.0.0`).
  - Multiple versions per commit: A commit can have multiple versions (e.g., `2026.s.0.1.23-b` for beta, then `2026.s.0.1.23` for stable) to mark a beta's transition to stable after verification.
  - Year in beta: A year version can remain in beta (e.g., `2026.b`) without reaching stable, moving to the next year's beta (e.g., `2027.b`) or stable/public beta (e.g., `2027.s`, `2027.p`) when ready.
- **Stability**: No version implies instability; MAJOR=0 indicates no breaking changes. Internal, beta, or `-b` releases may be unstable.
- Examples:
  - `i.0.32.102`: Internal phase, no breaking changes, 32 minor updates, 102 patches.
  - `2026.b.0.1.4`: Beta phase, no breaking changes, 1 minor update, 4 patches.
  - `2026.p.1.12.4-b`: Public beta phase, 1 breaking change, 12 minor updates, 4 patches, beta status.
  - `2026.s.2.3.4`: Stable phase, 2 breaking changes, 3 minor updates, 4 patches.
  - `2026a.s.0.0.0-p`: Stable phase, variant stream, no breaking changes, public beta status.

## Notes
- Recommended to use Git tags for versioning. Use `git tag -a` for annotated tags with the version as the tag name (e.g., `git tag -a v2026.s.1.0.0`).

## `version` CLI
- Rust-based tool for managing Git tags in the versioning format.
- **Commands**:
  - `version`: Prints the latest version tag (e.g., `v2026.s.2.3.4`) and copies the prefix to the clipboard: `vi.` for internal phase, `vYEAR[VARIANT].PHASE.` for other phases (e.g., `v2026.s.`, `v2026a.b.`).
  - `version help`: Prints the versioning overview for reference when manually entering tags.
- **Usage**: Run `version` in a Git repository.

## Future Considerations
- Enhanced `git` integration or a custom Git implementation to support advanced versioning workflows, enabling seamless management of multiple versions.
  - Example: Simultaneous development on new betas (e.g., 2027) and security updates for older releases (e.g., 2026).
- CLI commands to reduce manual `git tag` construction:
  - `version list`: Show version history, filterable by phase or variant.
  - `version bump [major|minor|patch]`: Auto-increment and tag based on the latest version.
  - `version new [phase] [--variant X]`: Transition to a new phase or variant, handling the `0.0.0` reset.
  - `version suffix [--add b|p] [--remove]`: Manage `-b`/`-p` suffixes for beta-to-stable promotion workflows.
- A small LLM instructions file for providing context on this versioning scheme to LLMs unfamiliar with it (e.g., for use in system prompts or project documentation).
