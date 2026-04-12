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

    // Apply core search/filter function
    let filtered = big_core::search::filter_entries(&entries, cmd.filter.as_deref());

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
