use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct UnpackCmd {
    /// Path to the .BIG archive
    pub archive: String,

    /// Output directory
    #[structopt(long, short = "o")]
    pub output: String,

    /// Show progress
    #[structopt(long)]
    pub progress: bool,
}

pub fn run(cmd: &UnpackCmd) -> anyhow::Result<()> {
    if cmd.progress {
        let (tx, rx) = big_core::progress::progress_channel();
        let archive = cmd.archive.clone();
        let output = cmd.output.clone();
        let handle = std::thread::spawn(move || {
            let _ = big_core::extract::extract_all(&archive, &output);
            let _ = tx.send(big_core::progress::Progress::Completed);
        });

        for ev in rx.iter() {
            eprintln!("[unpack] {:?}", ev);
        }

        let _ = handle.join();
    } else {
        big_core::extract::extract_all(&cmd.archive, &cmd.output)?;
    }

    println!("Unpacked {} -> {}", cmd.archive, cmd.output);
    Ok(())
}
