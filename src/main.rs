use eframe::{
    CreationContext, NativeOptions,
    egui::{self, Context, RichText, Ui, Vec2},
    run_native,
};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Default)]
struct AstroStackerApp {
    lights: Vec<PathBuf>,
    darks: Vec<PathBuf>,
    flats: Vec<PathBuf>,
    flat_darks: Vec<PathBuf>,
    biases: Vec<PathBuf>,
}

impl eframe::App for AstroStackerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(
                RichText::new("📷 FITS Stacker File Selector")
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(10.0);

            let button_size: Vec2 = egui::vec2(ui.available_width() * 0.5, 32.0);

            let mut pick_files = |label: &str, files: &mut Vec<PathBuf>| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label(format!("{} ({} files)", label, files.len()));
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(
                                RichText::new(format!("Select {label}")).size(16.0).strong(),
                            ),
                        )
                        .clicked()
                    {
                        if let Some(paths) = FileDialog::new()
                            .set_title(format!("Select {label} files"))
                            .pick_files()
                        {
                            *files = paths;
                        }
                    }
                });
                ui.add_space(5.0);
            };

            pick_files("Lights", &mut self.lights);
            pick_files("Darks", &mut self.darks);
            pick_files("Flats", &mut self.flats);
            pick_files("Flat Darks", &mut self.flat_darks);
            pick_files("Biases", &mut self.biases);

            ui.separator();
            ui.add_space(10.0);

            if ui
                .add_sized(
                    egui::vec2(ui.available_width(), 40.0),
                    egui::Button::new(
                        RichText::new("📤 Print Selected Files to Console")
                            .size(18.0)
                            .strong(),
                    ),
                )
                .clicked()
            {
                println!("Lights:");
                for light in &self.lights {
                    println!("  {}", light.display());
                }
                println!("Darks:");
                for dark in &self.darks {
                    println!("  {}", dark.display());
                }
                println!("Flats:");
                for flat in &self.flats {
                    println!("  {}", flat.display());
                }
                println!("Flat Darks:");
                for flat_dark in &self.flat_darks {
                    println!("  {}", flat_dark.display());
                }
                println!("Biases:");
                for bias in &self.biases {
                    println!("  {}", bias.display());
                }
            }
        });
    }
}

fn main() {
    let options: NativeOptions = NativeOptions::default();
    run_native(
        "Astro Stacker",
        options,
        Box::new(|_cc: &CreationContext<'_>| Ok(Box::new(AstroStackerApp::default()))),
    )
    .expect("Failed to start Astro Stacker application");
}
