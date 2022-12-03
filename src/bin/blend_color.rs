use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    back: String,

    #[arg(long)]
    fore: String,
}

fn main() {
    let args = Args::parse();

    let back = color::RGB::from_hex_str(&args.back).unwrap();
    let fore = color::RGB::from_hex_str(&args.fore).unwrap();

    println!(
        "{}",
        color::RGB {
            r: std::cmp::min(back.r, fore.r)
                + (std::cmp::max(back.r, fore.r) - std::cmp::min(back.r, fore.r)) / 2,
            g: std::cmp::min(back.g, fore.g)
                + (std::cmp::max(back.g, fore.g) - std::cmp::min(back.g, fore.g)) / 2,
            b: std::cmp::min(back.b, fore.b)
                + (std::cmp::max(back.b, fore.b) - std::cmp::min(back.b, fore.b)) / 2,
        }
        .to_hex_str()
    );
}
