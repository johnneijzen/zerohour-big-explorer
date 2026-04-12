use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct ListCmd {
    /// Path to the .BIG archive
    pub archive: String,

    /// Output JSON
    #[structopt(long)]
    pub json: bool,

    /// Filter by name substring
    #[structopt(long)]
    pub filter: Option<String>,
}

pub fn run(cmd: &ListCmd) -> anyhow::Result<()> {
    let (archive_meta, _index, entries) = big_core::parse_archive(&cmd.archive)?;

    // Apply simple substring filter if provided
    let filtered: Vec<_> = if let Some(ref f) = cmd.filter {
        entries.into_iter().filter(|e| e.name.contains(f)).collect()
    } else {
        entries
    };

    if cmd.json {
        let out = serde_json::to_string_pretty(&filtered)?;
        println!("{}", out);
    } else {
        println!("Archive: {} ({} bytes)", archive_meta.path, archive_meta.size);
        for e in filtered {
            println!("{}\t{} bytes\tcompressed={} ", e.name, e.length, e.compressed);
        }
    }

    Ok(())
}
