use eframe::{
    CreationContext, NativeOptions,
    egui::{self, Context, Ui},
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
            ui.heading("FITS Stacker File Selector");

            let mut pick_files = |label: &str, files: &mut Vec<PathBuf>| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label(format!("{} ({} files)", label, files.len()));
                    if ui.button(format!("Select {label}")).clicked() {
                        if let Some(paths) = FileDialog::new()
                            .set_title(format!("Select {label} files"))
                            .pick_files()
                        {
                            *files = paths;
                        }
                    }
                });
            };

            pick_files("Lights", &mut self.lights);
            pick_files("Darks", &mut self.darks);
            pick_files("Flats", &mut self.flats);
            pick_files("Flat Darks", &mut self.flat_darks);
            pick_files("Biases", &mut self.biases);

            ui.separator();

            if ui.button("Print Selected Files to Console").clicked() {
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
