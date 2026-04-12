use structopt::StructOpt;
use anyhow::Context;

#[derive(StructOpt, Debug)]
pub struct ExtractCmd {
    /// Path to the .BIG archive
    pub archive: String,

    /// Entry name to extract
    pub entry: String,

    /// Destination path for extracted file
    pub dest: String,

    /// Dry run (do not write files)
    #[structopt(long)]
    pub dry_run: bool,

    /// Output machine-readable JSON
    #[structopt(long)]
    pub json: bool,

    /// Preserve file permissions when extracting (placeholder)
    #[structopt(long)]
    pub preserve_permissions: bool,
}

pub fn run(cmd: &ExtractCmd) -> anyhow::Result<()> {
    let (_archive_meta, _index, entries) = big_core::parse_archive(&cmd.archive)?;

    let maybe = entries.into_iter().find(|e| e.name == cmd.entry);
    let entry = match maybe {
        Some(e) => e,
        None => return Err(anyhow::anyhow!("entry not found")),
    };

    if cmd.dry_run {
        if cmd.json {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({"action":"dry-run","entry":entry.name,"length":entry.length,"dest":cmd.dest}))?);
        } else {
            println!("Would extract '{}' ({} bytes) to {}", entry.name, entry.length, cmd.dest);
        }
        return Ok(());
    }
    // run extract with optional inline progress display if stdout is a tty or user asked for JSON false
    if cmd.json {
        match big_core::extract::extract_entry_to_path(&cmd.archive, &entry, &cmd.dest) {
            Ok(()) => println!("{}", serde_json::to_string_pretty(&serde_json::json!({"success":true,"entry":entry.name,"dest":cmd.dest}))?),
            Err(e) => println!("{}", serde_json::to_string_pretty(&serde_json::json!({"success":false,"error":format!("{}", e)}))?),
        }
        return Ok(());
    }

    // show progress inline
    let (tx, rx) = big_core::progress::progress_channel();
    let archive = cmd.archive.clone();
    let dest = cmd.dest.clone();
    let entry_clone = entry.clone();
    let handle = std::thread::spawn(move || {
        let _ = big_core::extract::extract_entry_to_path_with_progress(&archive, &entry_clone, &dest, Some(tx));
    });

    for ev in rx.iter() {
        eprintln!("[extract] {:?}", ev);
    }

    let res = handle.join();
    match res {
        Ok(_) => {
            println!("Extracted {} to {}", entry.name, cmd.dest);
            if cmd.preserve_permissions {
                eprintln!("Note: preserve-permissions is a placeholder in this scaffold");
            }
            Ok(())
        }
        Err(_) => Err(anyhow::anyhow!("extract thread panicked")),
    }
}
