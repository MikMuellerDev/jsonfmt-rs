use std::{env, fs, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Expected exactly 1 argument (filename), got {}",
            args.len() - 1
        );
        process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("Could not read file: {err}");
        process::exit(1);
    });

    let parsed: serde_json::Value = serde_json::from_str::<_>(&source).unwrap_or_else(|err| {
        eprintln!("Invalid JSON format in source: {err}");
        process::exit(2);
    });

    if let Err(err) = fs::write(
        &args[1],
        serde_json::to_string_pretty(&parsed).unwrap_or_else(|err| {
            eprintln!("Could not encode JSON: {err}");
            process::exit(3);
        }),
    ) {
        eprintln!("Could not write to {}: {err}", args[1]);
    }
}
