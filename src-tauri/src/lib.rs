// The Tauri iOS build invokes Cargo with `--lib`. Reuse the application
// implementation from main.rs so the library target exposes the same Tauri
// commands and module tree without duplicating the source.
include!("main.rs");
