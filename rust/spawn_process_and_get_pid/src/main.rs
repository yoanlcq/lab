use std::io::Write;

static HELP: &'static str = 
"spawn_process_and_get_pid, written by Yoan Lecoq; source available at https://github.com/yoanlcq/lab/tree/master/rust/spawn_process_and_get_pid
Because apparently, spawning a process and getting its ID via Windows batch scripts is hard...

To use, just specify the pass the full command you want to run as arguments; this will spawn a process and print its PID.

The exit code is given by the child process.
Because stdout is inherited from the child, you may want to pass \"-out_pid_file=pid.txt\" as first argument so that the PID is printed to a file, rather than stdout (which may get mixed up with the child's stdout).";


fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    let mut out_pid_file: Option<std::fs::File> = None;
    let mut has_errors = false;
    let mut cmd_args_start = 1;
    for i in 1..args.len() {
        if !args[i].starts_with("-") {
            cmd_args_start = i;
            break;
        }

        if args[i].starts_with("-out_pid_file=") {
            let tokens: Vec<_> = args[i].splitn(2, "=").collect();
            let path_str = tokens.get(1).unwrap_or_else(|| panic!("Empty file specified"));
            let path = std::path::PathBuf::from(path_str);
            out_pid_file = Some(std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(&path).unwrap_or_else(|e| panic!("Failed to open `{}`: {}", path.display(), e)));
        } else {
            has_errors = true;
            eprintln!("Unrecognized argument: \"{}\"", args[i]);
        }
    }

    if has_errors {
        std::process::exit(1);
    }

    if cmd_args_start >= args.len() {
        eprintln!("Please specify an executable to run");
        std::process::exit(1);
    }

    let mut cmd = std::process::Command::new(&args[cmd_args_start]);

    if cmd_args_start + 1 < args.len() {
        cmd.args(&args[cmd_args_start + 1 ..]);
    }

    let mut child = cmd.spawn().unwrap_or_else(|e| panic!("Failed to spawn process: {}", e));

    match out_pid_file.as_mut() {
        Some(out_pid_file) => write!(out_pid_file, "{}", child.id()).unwrap(),
        None => println!("{}", child.id()),
    };

    let exit_status = child.wait().unwrap_or_else(|e| panic!("Failed to wait process: {}", e));
    let exit_code = exit_status.code().unwrap_or_else(|| panic!("Failed to get exit code"));
    std::process::exit(exit_code)
}
