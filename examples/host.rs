fn main() {
    let host = rustc_host::from_cli().unwrap();
    println!("host: {}", host);
}
