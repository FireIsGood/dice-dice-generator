mod args;
mod config;
mod format;

use crate::config::get_config;

fn main() -> anyhow::Result<()> {
    get_config()?.print();

    Ok(())
}
