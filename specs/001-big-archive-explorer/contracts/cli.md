CLI Contract

Commands and behaviors (stable surface):

- `big-cli list <archive>`
  - Output: human table by default; `--json` emits JSON array of entries
  - Flags: `--json`, `--filter <pattern>`, `--verbose`

- `big-cli extract <archive> <outdir>`
  - Behavior: extracts safely into `outdir`; supports `--dry-run`, `--filter`, `--preserve-permissions`
  - Exit codes: `0` success, `1` warnings, `2` errors

- `big-cli pack <srcdir> <archive>`
  - Behavior: deterministic packing; options `--compress-level`, `--timestamp-policy`

- `big-cli validate <archive>`
  - Output: human summary; `--json` for machine parsing with structured `ValidationResult`

Error handling
- CLI must return clear exit codes and machine-readable JSON when `--json` provided.
