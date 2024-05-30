use egui::Align;
use rand::Rng;

#[derive(
    Debug, Copy, Clone, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Unit {
    #[default]
    Paragraphs,
    Words,
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LoremApp {
    #[serde(skip)]
    text: String,
    #[serde(skip)]
    length: usize,
    unit: Unit,
}

impl LoremApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for LoremApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.add(egui::DragValue::new(&mut self.length)).changed() {
                    self.generate()
                }

                self.unit_combo_box(ui).response.changed();

                ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                        self.copy_to_clipboard()
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| ui.label(self.text.clone()));
    }
}

impl LoremApp {
    fn unit_combo_box(
        &mut self,
        ui: &mut egui::Ui,
    ) -> egui::InnerResponse<std::option::Option<()>> {
        egui::ComboBox::from_id_source("unit_type")
            .selected_text(format!("{:?}", self.unit))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(&mut self.unit, Unit::Paragraphs, "Paragraphs")
                    .changed()
                {
                    self.generate()
                }
                if ui
                    .selectable_value(&mut self.unit, Unit::Words, "Words")
                    .changed()
                {
                    self.generate()
                };
            })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn copy_to_clipboard(&self) {
        use arboard::Clipboard;
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(self.text.clone()).unwrap();
    }

    #[cfg(target_arch = "wasm32")]
    fn copy_to_clipboard(&self) {
        use web_sys::window;

        let navigator = window().unwrap().navigator();

        let _ = navigator
            .clipboard()
            .unwrap()
            .write_text(self.text.as_str());
    }

    fn generate(&mut self) {
        self.text = match self.unit {
            Unit::Paragraphs => {
                let mut rng = rand::thread_rng();

                let texts: Vec<String> = (0..self.length)
                    .map(|i| {
                        let m = rng.gen_range(30..70);
                        match i {
                            0 => lipsum::lipsum(m),
                            _ => lipsum::lipsum_words(m),
                        }
                    })
                    .collect();

                texts.join("\n\n")
            }
            Unit::Words => lipsum::lipsum(self.length),
        };
    }
}
