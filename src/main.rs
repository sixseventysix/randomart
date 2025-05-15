fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => {
            if args.len() < 4 || args.len() > 7 {
                eprintln!("usage: {} generate <string> <depth> <width>(opt) <height>(opt) <outfile>(opt)", args[0]);
                std::process::exit(1);
            }
            let string = args[2].clone();
            let depth = args[3].parse().unwrap();
            let width = args.get(4).map_or(400, |v| v.parse().unwrap_or(400));
            let height = args.get(5).map_or(400, |v| v.parse().unwrap_or(400));
            let ns = args.get(6).cloned().unwrap_or_else(|| string.clone());

            randomart::RandomArtGenerate {
                string,
                depth,
                width,
                height,
                output_file_namespace: ns,
            }
            .run();
        }

        Some("read") => {
            if args.len() < 3 || args.len() > 6 {
                eprintln!("usage: {} read <input> <width>(opt) <height>(opt) <outfile>(opt)", args[0]);
                std::process::exit(1);
            }
            let input_file = args[2].clone();
            let width = args.get(3).map_or(400, |v| v.parse().unwrap_or(400));
            let height = args.get(4).map_or(400, |v| v.parse().unwrap_or(400));
            let ns = args.get(5).cloned().unwrap_or_else(|| input_file.clone());

            randomart::RandomArtRead {
                input_file,
                width,
                height,
                output_file_namespace: ns,
            }
            .run();
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("{} generate <string> <depth> <width>(opt) <height>(opt) <outfile>(opt)", args[0]);
            eprintln!("{} read <input> <width>(opt) <height>(opt) <outfile>(opt)", args[0]);
        }
    }
}