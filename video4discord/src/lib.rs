use std::io::{self, Write};
use std::process::{exit, Command, Output};

pub fn calculate_video_bitrate(
    video_duration: f32,
    target_filesize: f32,
    audio_bitrate: f32,
    muxing_overhead: f32,
) -> u32 {
    // muxing_overhead is a percentage of video+audio data filesize, not of final filesize
    // total_filesize = duration * (v_bitrate + a_bitrate) + mux_overhead  * duration * (v_bitrate + a_bitrate)
    let mux = muxing_overhead / 100.0;
    let total_filesize = target_filesize * 8192.0;
    (((total_filesize / video_duration) - (mux + 1.0) * audio_bitrate) / (mux + 1.0)) as u32
}

pub fn get_video_duration(input_file: &str) -> usize {
    let output = Command::new("ffprobe")
        .args(["-v", "error"])
        .args(["-select_streams", "v:0"])
        .args(["-show_entries", "format=duration"])
        .args(["-of", "default=noprint_wrappers=1:nokey=1"])
        .arg(input_file)
        .output()
        .expect("failed to execute process");
    exit_on_error(&output);

    String::from_utf8(output.stdout)
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap()
        + 1
}

pub struct AVOptions {
    pub audio_bitrate: u16,
    pub video_bitrate: u32,
    pub audio_codec: String,
}

pub fn run_ffmpeg(
    av_options: AVOptions,
    div: u8,
    target_filesize: f32,
    input_file: &str,
    output_file: &str,
) {
    let video_bitrate = format!("{}k", av_options.video_bitrate);
    let audio_bitrate = format!("{}k", av_options.audio_bitrate);
    let scale_filter = format!("scale=iw/{}:-1", div);

    let dev_null = if cfg!(target_os = "windows") {
        "NUL"
    } else {
        "/dev/null"
    };

    println!("aiming for filesize < {}MiB", target_filesize);
    println!("scaling video down to 1/{} x/y resolution", div);
    println!("new audio bitrate: {}", &audio_bitrate);
    println!("new video bitrate: {}", &video_bitrate);

    println!("running ffmpeg pass 1");
    let mut output = Command::new("ffmpeg")
        .arg("-y")
        .args(["-i", input_file])
        .args(["-c:v", "libx264"])
        .args(["-b:v", &video_bitrate])
        .args(["-pass", "1"])
        .args(["-vf", &scale_filter])
        .args(["-vsync", "cfr"])
        .args(["-f", "null"])
        .arg(dev_null)
        .output()
        .expect("failed to execute process");
    exit_on_error(&output);

    println!("running ffmpeg pass 2");
    output = Command::new("ffmpeg")
        .arg("-y")
        .args(["-i", input_file])
        .args(["-c:v", "libx264"])
        .args(["-b:v", &video_bitrate])
        .args(["-pass", "2"])
        .args(["-vf", &scale_filter])
        .args(["-c:a", &av_options.audio_codec])
        .args(["-b:a", &audio_bitrate])
        .arg(output_file)
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
