fn main() -> Result<(), rustc_host::Error> {
    println!("{}", rustc_host::from_cli()?);
    Ok(())
}