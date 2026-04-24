use structopt::StructOpt;

mod commands;
use commands::append::{AppendCmd, run as run_append};
use commands::extract::{ExtractCmd, run as run_extract};
use commands::list::{ListCmd, run as run_list};
use commands::pack::{PackCmd, run as run_pack};
use commands::unpack::{UnpackCmd, run as run_unpack};
use commands::validate::{ValidateCmd, run as run_validate};

#[derive(StructOpt, Debug)]
#[structopt(name = "big-cli", about = "ZeroHour BIG archive CLI")]
enum Cmd {
    /// List archive contents
    List(ListCmd),
    /// Extract an entry from the archive
    Extract(ExtractCmd),
    /// Pack a directory into an archive
    Pack(PackCmd),
    /// Unpack an archive to a directory
    Unpack(UnpackCmd),
    /// Append a file into an archive
    Append(AppendCmd),
    /// Validate an archive
    Validate(ValidateCmd),
}

fn main() -> anyhow::Result<()> {
    match Cmd::from_args() {
        Cmd::List(list_cmd) => run_list(&list_cmd)?,
        Cmd::Extract(extract_cmd) => run_extract(&extract_cmd)?,
        Cmd::Pack(pack_cmd) => run_pack(&pack_cmd)?,
        Cmd::Unpack(unpack_cmd) => run_unpack(&unpack_cmd)?,
        Cmd::Append(append_cmd) => run_append(&append_cmd)?,
        Cmd::Validate(validate_cmd) => run_validate(&validate_cmd)?,
    }
    Ok(())
}
