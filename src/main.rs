mod app;
mod crossterm;
mod ui;
mod error;
use crate::crossterm::run;
use std::time::Duration;

fn main() -> error::IResult<()> {
    let tick_rate = Duration::from_millis(250);
    run(tick_rate, true)?;

    Ok(())
}
