static HELP: &'static str = 
"file_date_tool, written by Yoan Lecoq; source available at https://github.com/yoanlcq/lab/tree/master/rust/file_date_tool

Commands are:
- write_timestamp <timestamp.txt>
  This writes the current timestamp to the specified file. The format is unspecified, but can be read later by this tool.
- filter_newer <timestamp.txt> <filelist.txt>
  Rewrite <filelist.txt> such that, for each file in <filelist.txt> (1 line = 1 file path), only the entries newer than the timestamp written in <timestamp.txt> are preserved.
  \"newer\" here is according to the last write time of each file.
";

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "write_timestamp" => {
            if args.len() <= 2 {
                eprintln!("{}", HELP);
                std::process::exit(1);
            }
            let date_file_path = &args[2];

            let system_time = std::time::SystemTime::now();
            let micros = system_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
            std::fs::write(date_file_path, &format!("{}", micros)).unwrap_or_else(|e| { eprintln!("Failed to write to `{}`: {}", date_file_path, e); std::process::exit(1) });
        },
        "filter_newer" => {
            if args.len() <= 3 {
                eprintln!("{}", HELP);
                std::process::exit(1);
            }
            let date_file_path = &args[2];
            let filelist_path = &args[3];

            let ref_date_str = std::fs::read_to_string(date_file_path).unwrap_or_else(|e| { eprintln!("Failed to open `{}`: {}", date_file_path, e); std::process::exit(1) });
            let ref_micros: u128 = ref_date_str.parse().expect("Failed to parse timestamp");

            let filelist = std::fs::read_to_string(filelist_path).unwrap_or_else(|e| { eprintln!("Failed to open `{}`: {}", filelist_path, e); std::process::exit(1) });

            // Try to respect line endings, but don't bother too much
            let newline = if filelist.contains("\r\n") { "\r\n" } else { "\n" };

            let mut changed = false;
            let mut new_filelist = String::with_capacity(filelist.len());
            for line in filelist.lines() {
                let file_metadata = std::fs::metadata(line).unwrap_or_else(|e| { eprintln!("Failed to stat `{}`: {}", line, e); std::process::exit(1) });
                let file_modified_system_time = file_metadata.modified().unwrap_or_else(|e| { eprintln!("Failed to get last modification time from `{}`: {}", line, e); std::process::exit(1) });
                let file_micros = file_modified_system_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
                if file_micros >= ref_micros {
                    new_filelist += line;
                    new_filelist += newline;
                } else {
                    changed = true;
                }
            }
            if changed {
                std::fs::write(filelist_path, new_filelist).unwrap_or_else(|e| { eprintln!("Failed to write to `{}`: {}", filelist_path, e); std::process::exit(1) });
            }
        },
        _ => {
            eprintln!("{}", HELP);
            std::process::exit(1);
        },
    }
}
