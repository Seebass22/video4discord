use clap::Parser;
use std::io::{self, Write};
use std::process::{exit, Command, Output};

/// Reencode a video to be under 8MiB
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 64)]
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
    output_file: String,
}

fn main() {
    let args = Args::parse();

    println!("bitrate: {}", args.audio_bitrate);
    println!("div: {}", args.div);
    println!("target filesize: {}", args.target_filesize);

    let bitrate = "400k";
    let audio_bitrate = format!("{}k", args.audio_bitrate);
    let scale = format!("scale=iw/{}:-1", args.div);

    let dev_null = if cfg!(target_os = "windows") {
        "NUL"
    } else {
        "/dev/null"
    };

    // pass 1
    println!("running ffmpeg pass 1");
    let mut output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&args.input_file)
        .arg("-c:v")
        .arg("libx264")
        .arg("-b:v")
        .arg(bitrate)
        .arg("-pass")
        .arg("1")
        .arg("-vf")
        .arg(&scale)
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
        .arg(bitrate)
        .arg("-pass")
        .arg("2")
        .arg("-vf")
        .arg(&scale)
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg(&audio_bitrate)
        .arg("output.mp4")
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
