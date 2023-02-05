fn main() -> Result<(), rustc_host::Error> {
    print!("{}", rustc_host::from_cli()?);
    Ok(())
}