fn main() -> battery::Result<()> {
    println!(
        "Batteries detected: {}",
        battery::Manager::new()?.batteries()?.count()
    );
    Ok(())
}
