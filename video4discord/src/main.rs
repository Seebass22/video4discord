use clap::{ArgEnum, Parser};
use video4discord::*;

/// Reencode a video to be under 8MiB
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// audio bitrate in Kbps
    #[clap(short, long, default_value_t = 64)]
    audio_bitrate: u16,

    /// audio codec: opus is more efficient, aac has better compatibility
    #[clap(arg_enum, short = 'c', long, default_value_t = AudioCodec::Opus)]
    audio_codec: AudioCodec,

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum AudioCodec {
    Opus,
    Aac,
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

    let audio_codec = match args.audio_codec {
        AudioCodec::Opus => "libopus",
        AudioCodec::Aac => "aac",
    };

    run_ffmpeg(
        AVOptions {
            audio_bitrate: args.audio_bitrate,
            video_bitrate,
            audio_codec: audio_codec.to_owned(),
        },
        args.div,
        args.target_filesize,
        &args.input_file,
        &args.output_file,
    );
}
