use std::path::PathBuf;

use video4discord::*;

pub struct GUI {
    input_file: Option<String>,
    output_file: String,
    output_folder: String,
    audio_bitrate: u16,
    muxing_overhead: f32,
    target_filesize: f32,
    div: u8,
    audio_codec: String,
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            input_file: None,
            output_file: "".to_owned(),
            output_folder: "".to_owned(),
            audio_bitrate: 64,
            muxing_overhead: 5.0,
            target_filesize: 8.0,
            div: 2,
            audio_codec: "libopus".to_owned(),
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
            ui.horizontal(|ui| {
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
            });

            ui.horizontal(|ui| {
                if ui.button("choose output folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.output_folder = path.display().to_string();
                    }
                }

                if self.output_folder != "" {
                    ui.label(&self.output_folder);
                } else {
                    ui.label("no folder selected");
                }
            });

            ui.add(
                egui::Slider::new(&mut self.audio_bitrate, 6..=128).text("audio bitrate in Kbps"),
            );
            ui.add(
                egui::Slider::new(&mut self.muxing_overhead, 1.0..=30.0)
                    .text("expected muxing overhead %"),
            );

            ui.horizontal(|ui| {
                ui.label("Divide X/Y resolution by:");
                for value in [1, 2, 4, 6, 10] {
                    ui.selectable_value(&mut self.div, value, value.to_string());
                }
            });

            ui.horizontal(|ui| {
                ui.label("audio codec:");
                for codec in ["aac", "libopus"] {
                    ui.selectable_value(&mut self.audio_codec, codec.to_owned(), codec.to_owned());
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

                    let output_file = if self.output_file == "" {
                        add_underscore(self.input_file.as_ref().unwrap())
                    } else {
                        self.output_file.clone()
                    };

                    let output_path = [&self.output_folder, &output_file]
                        .iter()
                        .collect::<PathBuf>();

                    run_ffmpeg(
                        AVOptions {
                            audio_bitrate: self.audio_bitrate,
                            video_bitrate,
                            audio_codec: self.audio_codec.clone(),
                        },
                        self.div,
                        self.target_filesize,
                        &input_file,
                        output_path.to_str().expect("path contains invalid unicode"),
                    );
                }
            }

            preview_files_being_dropped(ctx);
            if !ctx.input().raw.dropped_files.is_empty() {
                let file = ctx.input().raw.dropped_files[0].clone();
                let info = if let Some(path) = &file.path {
                    Some(path.display().to_string())
                } else if !file.name.is_empty() {
                    Some(file.name.clone())
                } else {
                    None
                };
                self.input_file = info;
            }
        });
    }
}

// https://github.com/emilk/egui/blob/218d4d4eeaa7f3c8495881d5e6555b14d8ab95eb/examples/file_dialog/src/main.rs
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
