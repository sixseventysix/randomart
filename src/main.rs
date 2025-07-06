fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => {
            if args.len() != 4 {
                eprintln!("incorrect num args")
                std::process::exit(1);
            }
            let string = args[2].clone();
            let depth = args[3].parse().unwrap();

            randomart::RandomArtGenerateCtx {
                string,
                depth,
            }
            .run();
        }

        Some("read") => {
            if args.len() != 3 {
                eprintln!("incorrect num args")
                std::process::exit(1);
            }
            let input_file = args[2].clone();

            randomart::RandomArtReadCtx {
                input_file,
            }
            .run();
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("{} generate <string> <depth>", args[0]);
            eprintln!("{} read <input>", args[0]);
        }
    }
}