use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct ValidateCmd {
    /// Path to the archive to validate
    pub archive: String,
    /// Output machine-readable JSON
    #[structopt(long)]
    pub json: bool,
}

pub fn run(cmd: &ValidateCmd) -> anyhow::Result<()> {
    let result = big_core::validate::validate_archive(&cmd.archive)?;
    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("Validation: errors={} warnings={}", result.errors.len(), result.warnings.len());
    }
    Ok(())
}
