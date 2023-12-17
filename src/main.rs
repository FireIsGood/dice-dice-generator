mod args;
mod config;
mod format;

use crate::config::get_config;

fn main() -> anyhow::Result<()> {
    let rules = "\
    ## Dice Dice (Day 1984)\n\
    \n\
    *Test Edition*\n\
    \n\
    Unspoiler any number of the following prompts, when you want to stop, record your number. \
    You may then check the rest. (please don't post the answers)\n\
    \n\
    You start with 100 points. Calculate on your points with what is written.\n\
    ";
    get_config()?.print(rules);

    Ok(())
}
