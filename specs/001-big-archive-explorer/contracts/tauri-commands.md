Tauri commands (Rust -> frontend RPC contract)

- `list_archive`
  - Request: `{ path: string }`
  - Response: `{ entries: Entry[] }`

- `extract_entry`
  - Request: `{ archive: string, entry: string, dest: string }`
  - Response: `{ success: bool, detail?: string }
`

- `pack_directory`
  - Request: `{ src: string, dest: string, options?: PackOptions }`
  - Response: `{ job_id: string }` (supports progress notifications)

- `validate_archive`
  - Request: `{ path: string }`
  - Response: `ValidationResult`

Notes: commands must be async-friendly and not block the Tauri UI thread; use streaming channels for large payloads.
