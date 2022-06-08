use video4discord::*;

pub struct GUI {
    // Example stuff:
    input_file: String,
    output_file: String,
    audio_bitrate: u16,
    muxing_overhead: f32,
    target_filesize: f32,
    div: u8,
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            input_file: "input.mp4".to_owned(),
            output_file: "output.mp4".to_owned(),
            audio_bitrate: 32,
            muxing_overhead: 5.0,
            target_filesize: 8.0,
            div: 2,
        }
    }
}

impl GUI {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for GUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("run").clicked() {
                let duration = get_video_duration(&self.input_file);
                let video_bitrate = calculate_video_bitrate(
                    duration as f32,
                    self.target_filesize,
                    self.audio_bitrate as f32,
                    self.muxing_overhead,
                );
                run_ffmpeg(
                    self.audio_bitrate,
                    video_bitrate,
                    self.div,
                    self.target_filesize,
                    &self.input_file,
                    &self.output_file,
                );
            }
        });
    }
}
