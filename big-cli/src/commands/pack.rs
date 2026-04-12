use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct PackCmd {
    /// Source directory to pack
    pub src: String,

    /// Destination archive path
    pub dest: String,
    /// Output machine-readable JSON
    #[structopt(long)]
    pub json: bool,
    /// Show progress in the terminal
    #[structopt(long)]
    pub progress: bool,
}

pub fn run(cmd: &PackCmd) -> anyhow::Result<()> {
    // call real packer, optionally showing progress
    if cmd.progress {
        let (tx, rx) = big_core::progress::progress_channel();
        let src = cmd.src.clone();
        let dest = cmd.dest.clone();
        let handle = std::thread::spawn(move || {
            let _ = big_core::pack::pack_directory_with_progress(&src, &dest, Some(tx));
        });

        for ev in rx.iter() {
            eprintln!("[pack] {:?}", ev);
        }

        let _ = handle.join();
    } else {
        big_core::pack::pack_directory(&cmd.src, &cmd.dest)?;
    }

    if cmd.json {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"success":true,"src":cmd.src,"dest":cmd.dest})
            )?
        );
    } else {
        println!("Packed {} -> {}", cmd.src, cmd.dest);
    }
    Ok(())
}
