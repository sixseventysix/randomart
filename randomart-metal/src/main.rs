use randomart_metal::{RandomArtGenerateCtx, RandomArtReadCtx};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => {
            if args.len() < 4 || args.len() > 6 {
                eprintln!("usage: {} generate <string> <depth> [width] [height]", args[0]);
                std::process::exit(1);
            }
            let string = args[2].clone();
            let depth = args[3].parse().unwrap();
            let width  = args.get(4).map_or(512, |v| v.parse().unwrap_or(512));
            let height = args.get(5).map_or(512, |v| v.parse().unwrap_or(512));

            RandomArtGenerateCtx { string, depth, width, height }.run();
        }

        Some("read") => {
            if args.len() < 3 || args.len() > 5 {
                eprintln!("usage: {} read <input> [width] [height]", args[0]);
                std::process::exit(1);
            }
            let input_filepath = args[2].clone();
            let width  = args.get(3).map_or(512, |v| v.parse().unwrap_or(512));
            let height = args.get(4).map_or(512, |v| v.parse().unwrap_or(512));

            RandomArtReadCtx { input_filepath, width, height }.run();
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("  {} generate <string> <depth> [width] [height]", args[0]);
            eprintln!("  {} read <input> [width] [height]", args[0]);
        }
    }
}
