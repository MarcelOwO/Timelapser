use eframe::egui;

struct Window {
    picker: Box<dyn FnMut()>,
}

impl Window {
    fn new<F>(cc: &eframe::CreationContext<'_>, picker: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            picker: Box::new(picker),
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timelapser");
            ui.spacing();
            if ui.button("Select Folder").clicked() {
                (*self.picker)();
            }
        });
    }
}

pub(crate) fn setup_window(f: impl FnMut() + 'static) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Timelapser",
        native_options,
        Box::new(|cc| Ok(Box::new(Window::new(cc, f)))),
    );
}
