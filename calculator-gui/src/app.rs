use eframe::{egui, epi};
use eframe::egui::{Color32, Key};
use calculator::calculate;

pub struct CalculatorApp {
	expression: String,
	output: Result<f64, String>
}

impl Default for CalculatorApp {
	fn default() -> Self {
		Self {
			expression: "0".to_owned(),
			output: Ok(0.0)
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
					*output =  calculate(expression);
				}
			});

			if ctx.input().keys_down.contains(&Key::Enter) {
				*output =  calculate(expression);
			}

			match output {
				Ok(n) => ui.label(format!("= {}", n)),
				Err(e) => ui.colored_label(Color32::from_rgb(255, 0, 0), format!("{}", e))
			};

			ui.label("You can use these operators: +, -, *, /, ^.");
			ui.label("These functions: sqrt, sin, cos, tan, abs, round, factorial.");
			ui.label("And these constants: pi, e.")
		});
	}

	fn name(&self) -> &str {
		"Calculator"
	}
}
