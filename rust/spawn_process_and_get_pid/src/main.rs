fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let mut child = std::process::Command::new(&args[1]).args(&args[2..]).spawn().unwrap_or_else(|e| panic!("Failed to spawn process: {}", e));
    println!("{}", child.id());
    let exit_status = child.wait().unwrap_or_else(|e| panic!("Failed to wait process: {}", e));
    let exit_code = exit_status.code().unwrap_or_else(|| panic!("Failed to get exit code"));
    std::process::exit(exit_code)
}
