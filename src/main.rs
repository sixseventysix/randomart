fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => {
            let string = args[2].clone();
            let depth = args[3].parse().unwrap();

            randomart::RandomArtGenerate {
                string,
                depth,
            }
            .run();
        }

        Some("read") => {
            let input_file = args[2].clone();

            randomart::RandomArtRead {
                input_file,
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