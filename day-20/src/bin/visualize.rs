use day_20::visualize::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input1.txt");
    process(file).context("visualize")?;
    Ok(())
}
