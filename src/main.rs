use clap::Parser;
use std::io::{self, Write};
use std::process::{exit, Command, Output};

/// Reencode a video to be under 8MiB
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 32)]
    audio_bitrate: u16,

    /// factor to divide X/Y resolution by
    #[clap(short, long, default_value_t = 2)]
    div: u8,

    /// target filesize in MiB
    #[clap(short, long, default_value_t = 7.999)]
    target_filesize: f32,

    #[clap(short)]
    input_file: String,

    #[clap(short, long, required = false)]
    output_file: Option<String>,
}

fn calculate_video_bitrate(
    video_duration: usize,
    target_filesize: f32,
    audio_bitrate: u16,
) -> usize {
    let total_bitrate = (target_filesize * 0.9536 * 8192.0) / video_duration as f32;
    (total_bitrate - audio_bitrate as f32) as usize
}

fn main() {
    let args = Args::parse();

    let video_duration = get_video_duration(&args.input_file);
    let video_bitrate = calculate_video_bitrate(video_duration, args.target_filesize, args.audio_bitrate);
    let video_bitrate = format!("{}k", video_bitrate);

    let audio_bitrate = format!("{}k", args.audio_bitrate);

    let output_file = args.output_file.unwrap_or("out.mp4".to_owned());

    let scale_filter = format!("scale=iw/{}:-1", args.div);

    let dev_null = if cfg!(target_os = "windows") {
        "NUL"
    } else {
        "/dev/null"
    };

    println!("aiming for filesize < {}MiB", &args.target_filesize);
    println!("scaling video down to 1/{} x/y resolution", &args.div);
    println!("new audio bitrate: {}", &audio_bitrate);
    println!("new video bitrate: {}", &video_bitrate);

    println!("running ffmpeg pass 1");
    let mut output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&args.input_file)
        .arg("-c:v")
        .arg("libx264")
        .arg("-b:v")
        .arg(&video_bitrate)
        .arg("-pass")
        .arg("1")
        .arg("-vf")
        .arg(&scale_filter)
        .arg("-vsync")
        .arg("cfr")
        .arg("-f")
        .arg("null")
        .arg(dev_null)
        .output()
        .expect("failed to execute process");
    exit_on_error(&output);

    println!("running ffmpeg pass 2");
    output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&args.input_file)
        .arg("-c:v")
        .arg("libx264")
        .arg("-b:v")
        .arg(&video_bitrate)
        .arg("-pass")
        .arg("2")
        .arg("-vf")
        .arg(&scale_filter)
        .arg("-c:a")
        .arg("libopus")
        .arg("-b:a")
        .arg(&audio_bitrate)
        .arg(&output_file)
        .output()
        .expect("failed to execute process");
    exit_on_error(&output);
}

fn exit_on_error(output: &Output) {
    if !output.status.success() {
        io::stdout().write_all(&output.stderr).unwrap();
        exit(-1);
    }
}

fn get_video_duration(input_file: &str) -> usize {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(input_file)
        .output()
        .expect("failed to execute process");
    exit_on_error(&output);

    String::from_utf8(output.stdout)
        .unwrap()
        .split(".")
        .next()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap()
        + 1
}
