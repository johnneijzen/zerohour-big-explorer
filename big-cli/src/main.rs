use structopt::StructOpt;

mod commands;
use commands::list::{ListCmd, run as run_list};

#[derive(StructOpt, Debug)]
#[structopt(name = "big-cli", about = "ZeroHour BIG archive CLI")]
enum Cmd {
    /// List archive contents
    List(ListCmd),
}

fn main() -> anyhow::Result<()> {
    match Cmd::from_args() {
        Cmd::List(list_cmd) => run_list(&list_cmd)?,
    }
    Ok(())
}
