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

		// Examples of how to create different panels and windows.
		// Pick whichever suits you.
		// Tip: a good default choice is to just keep the `CentralPanel`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		/*egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if ui.button("Quit").clicked() {
						frame.quit();
					}
				});
			});
		});

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.heading("Side Panel");

			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(label);
			});

			ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				*value += 1.0;
			}

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				ui.horizontal(|ui| {
					ui.spacing_mut().item_spacing.x = 0.0;
					ui.label("powered by ");
					ui.hyperlink_to("egui", "https://github.com/emilk/egui");
					ui.label(" and ");
					ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's

			ui.heading("eframe template");
			ui.hyperlink("https://github.com/emilk/eframe_template");
			ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
			egui::warn_if_debug_build(ui);
		});

		if true {
			egui::Window::new("Window").show(ctx, |ui| {
				ui.label("Windows can be moved by dragging them.");
				ui.label("They are automatically sized based on contents.");
				ui.label("You can turn on resizing and scrolling if you like.");
				ui.label("You would normally chose either panels OR windows.");
			});
		}*/
	}

	fn name(&self) -> &str {
		"Calculator"
	}
}
