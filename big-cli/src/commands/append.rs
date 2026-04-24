use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct AppendCmd {
    /// Path to the .BIG archive
    pub archive: String,

    /// Source file on disk to append
    pub source: String,

    /// Path inside archive to store the file
    #[structopt(long = "path")]
    pub path: String,

    /// Overwrite existing entry
    #[structopt(long)]
    pub force: bool,

    /// Output machine-readable JSON
    #[structopt(long)]
    pub json: bool,
}

pub fn run(cmd: &AppendCmd) -> anyhow::Result<()> {
    let res =
        big_core::pack::append_file_to_archive(&cmd.archive, &cmd.source, &cmd.path, cmd.force);
    match res {
        Ok(()) => {
            if cmd.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({"success":true,"archive":cmd.archive,"path":cmd.path})
                    )?
                );
            } else {
                println!("Appended {} -> {}", cmd.source, cmd.archive);
            }
            Ok(())
        }
        Err(e) => {
            if cmd.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({"success":false,"error":format!("{}", e)})
                    )?
                );
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
