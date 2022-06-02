use clap::Parser;

/// Reencode a video to be under 8MB
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// audio bitrate
    #[clap(short, long, default_value_t = 64)]
    audio_bitrate: u16,

    /// factor to divide X/Y resolution by
    #[clap(short, long, default_value_t = 2)]
    div: u8,

    /// target filesize
    #[clap(short, long, default_value_t = 7.999)]
    target_filesize: f32,
}

fn main() {
    let args = Args::parse();

    println!("bitrate: {}", args.audio_bitrate);
    println!("div: {}", args.div);
    println!("target filesize: {}", args.target_filesize);
}
