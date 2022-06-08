use video4discord::*;

pub struct GUI {
    input_file: Option<String>,
    output_file: String,
    audio_bitrate: u16,
    muxing_overhead: f32,
    target_filesize: f32,
    div: u8,
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            input_file: None,
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
            if ui.button("choose file").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_file = Some(path.display().to_string());
                }
            }


            if let Some(input_file) = &self.input_file {
                ui.label(input_file);
            } else {
                ui.label("no file selected");
            }

            ui.add(egui::Slider::new(&mut self.audio_bitrate, 6..=128).text("audio bitrate in Kbps (opus codec)"));
            ui.add(egui::Slider::new(&mut self.muxing_overhead, 1.0..=30.0).text("expected muxing overhead %"));

            ui.label("Divide X/Y resolution by:");
            ui.horizontal(|ui| {
                for value in [1, 2, 4, 6, 10] {
                    if ui.selectable_value(&mut self.div, value, value.to_string()).clicked() {
                    }
                }
            });


            if ui.button("run").clicked() {
                if let Some(input_file) = &self.input_file {
                    let duration = get_video_duration(&input_file);
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
                        &input_file,
                        &self.output_file,
                    );
                }
            }
        });
    }
}
