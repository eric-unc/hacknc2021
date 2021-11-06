use eframe::{egui, epi};
use calculator::calculate;

pub struct CalculatorApp {
	expression: String,
	output: String
}

impl Default for CalculatorApp {
	fn default() -> Self {
		Self {
			expression: "0".to_owned(),
			output: "0".to_owned()
		}
	}
}

impl epi::App for CalculatorApp {
	fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
		let Self { expression, output } = self;

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::widgets::global_dark_light_mode_switch(ui);

				if ui.button("Quit").clicked() {
					frame.quit();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Calculator");
			ui.label("Put your expression here: ");

			ui.horizontal(|ui| {
				ui.text_edit_singleline(expression);
				if ui.button("Evaluate").clicked() {
					match calculate(expression) {
						Ok(n) => {
							*output = n.to_string();
						}
						Err(e) => {
							*output = e;
						}
					}
				}
			});

			ui.label(format!("= {}", output));
		});
	}

	fn name(&self) -> &str {
		"Calculator"
	}
}
