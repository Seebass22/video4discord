use clap::Parser;
use video4discord::*;

/// Reencode a video to be under 8MiB
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// audio bitrate in Kbps
    #[clap(short, long, default_value_t = 32)]
    audio_bitrate: u16,

    /// factor to divide X/Y resolution by
    #[clap(short, long, default_value_t = 2)]
    div: u8,

    /// target filesize in MiB
    #[clap(short, long, default_value_t = 8.0)]
    target_filesize: f32,

    /// muxing overhead in percent
    #[clap(short, long, default_value_t = 5.0)]
    muxing_overhead: f32,

    #[clap(short)]
    input_file: String,

    #[clap(short, default_value_t = String::from("out.mp4"))]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    let video_duration = get_video_duration(&args.input_file);
    let video_bitrate = calculate_video_bitrate(
        video_duration as f32,
        args.target_filesize,
        args.audio_bitrate as f32,
        args.muxing_overhead,
    );

    run_ffmpeg(
        args.audio_bitrate,
        video_bitrate,
        args.div,
        args.target_filesize,
        &args.input_file,
        &args.output_file,
    );
}
