mod db;
use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust + Supabase",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    input_text: String,
    status: String,
    data: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust + Supabase 🦀");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Input: ");
                ui.text_edit_singleline(&mut self.input_text);
            });

            if ui.button("Send to Supabase").clicked() {
                let text = self.input_text.clone();
                let data = Arc::clone(&self.data);
                
                tokio::spawn(async move {
                    let client = db::get_client();
                    let body = format!(r#"{{"message": "{}"}}"#, text);
                    
                    let _resp = client
                        .from("messages")
                        .insert(&body)
                        .execute()
                        .await;

                    let mut d = data.lock().unwrap();
                    d.push(text);
                });

                self.status = "Sent!".to_string();
                self.input_text.clear();
            }

            ui.label(&self.status);

            ui.separator();
            ui.label("Messages:");
            let data = self.data.lock().unwrap();
            for msg in data.iter() {
                ui.label(format!("• {}", msg));
            }
        });
    }
}
