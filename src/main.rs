mod day1;
mod day2;

fn main() -> Result<(), failure::Error> {
    day1::day1_first_run()?;
    day1::day1_second_run()?;
    day2::day2_first_run()?;
    Ok(())
}
