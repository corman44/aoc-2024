use day_20::part2::process;

#[tracing::instrument]
fn main() -> Result<(), String> {

    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}