use eframe::egui::{self, Vec2, Pos2};
use std::collections::HashMap;

struct Rect {
    id: String,
    position: (f32, f32),
    target_position: (f32, f32),
    animating: bool,
}

struct MyEguiApp {
    rects: HashMap<String, Rect>,
    selected_id1: String,
    x_input1: String,
    y_input1: String,
    selected_id2: String,
    x_input2: String,
    y_input2: String,
    step: f32,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        let mut rects = HashMap::new();
        rects.insert("1".to_string(), Rect {
            id: "1".to_string(), position: (100.0, 100.0),
            target_position: (100.0, 100.0), animating: false,
        });
        rects.insert("2".to_string(), Rect {
            id: "2".to_string(), position: (200.0, 200.0),
            target_position: (100.0, 100.0), animating: false,
        });
        Self {
            rects,
            selected_id1: "1".to_string(),
            x_input1: "".to_string(),
            y_input1: "".to_string(),
            selected_id2: "2".to_string(),
            x_input2: "".to_string(),
            y_input2: "".to_string(),
            step: 0.1f32, // Adjust this value for animation speed
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for rect in self.rects.values_mut() {
                if rect.animating {
                    let dx = rect.target_position.0 - rect.position.0;
                    let dy = rect.target_position.1 - rect.position.1;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 1.0 {
                        rect.position = rect.target_position;
                        rect.animating = false;
                    } else {
                        rect.position.0 += dx * self.step;
                        rect.position.1 += dy * self.step;
                    }
                }

                // Draw the rectangle at its current position
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        Pos2::new(rect.position.0, rect.position.1),
                        Vec2::new(50.0, 50.0),
                    ),
                    0.0,
                    egui::Color32::from_rgb(150, 150, 150),
                );
            }

            // Interaction
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("ID1:");
                    ui.text_edit_singleline(&mut self.selected_id1);
                    ui.label("X1:");
                    ui.text_edit_singleline(&mut self.x_input1);
                    ui.label("Y1:");
                    ui.text_edit_singleline(&mut self.y_input1);
                });
                ui.horizontal(|ui| {
                    ui.label("ID2:");
                    ui.text_edit_singleline(&mut self.selected_id2);
                    ui.label("X2:");
                    ui.text_edit_singleline(&mut self.x_input2);
                    ui.label("Y2:");
                    ui.text_edit_singleline(&mut self.y_input2);
                });
                ui.horizontal(|ui| {
                    if ui.button("Move").clicked() {
                        if let (Ok(x), Ok(y)) = (self.x_input1.parse::<f32>(), self.y_input1.parse::<f32>()) {
                            if let Some(rect) = self.rects.get_mut(&self.selected_id1) {
                                rect.target_position = (x, y);
                                rect.animating = true;
                            }
                        }
                        if let (Ok(x), Ok(y)) = (self.x_input2.parse::<f32>(), self.y_input2.parse::<f32>()) {
                            if let Some(rect) = self.rects.get_mut(&self.selected_id2) {
                                rect.target_position = (x, y);
                                rect.animating = true;
                            }
                        }
                    }
                });
                ui.horizontal(|ui| {
                    let slider_response = ui.add(
                        egui::Slider::new(&mut self.step, 0.0..=0.1)
                            .logarithmic(true)
                            .text("Speed"),
                    );
                    if slider_response.changed() {
                        //
                    }
                });
            });
        });

        // Request repaint if any rectangle is animating
        if self.rects.values().any(|r| r.animating) {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Animation Example",
        options,
        Box::new(|_cc| Box::new(MyEguiApp::default())),
    )
}
