use chrono::Local;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TestApp {
    username: String,

    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    password: String,

    #[serde(skip)]
    logged_in: bool,
}

impl Default for TestApp {
    /// Default the Test Application.
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
            logged_in: false,
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TestApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

pub fn yay_if_release_build(ui: &mut egui::Ui) {
    if !cfg!(debug_assertions) {
        ui.label(
            egui::RichText::new("** Release Build **")
                .small()
                .color(egui::Color32::GREEN),
        )
        .on_hover_text("egui was compiled for production.");
    }
}

impl eframe::App for TestApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            username,
            password,
            logged_in,
            label,
            value,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Filf", |ui| {
                    if ui.button("Logout").clicked() {
                        *password = "".to_owned();
                        *logged_in = false;
                    }

                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if *logged_in {
                    ui.label(format!("{} is Logged In.", username));

                    let now = Local::now().to_rfc2822();
                    ui.centered_and_justified(|ui| ui.label(now.clone()));

                    //if now != *last_time {
                    //ctx.request_repaint();
                    //}
                } else {
                    ui.label("Logged Out.");
                }
            })
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
            yay_if_release_build(ui);
        });

        if !*logged_in {
            //egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            //    egui::warn_if_debug_build(ui);
            //    yay_if_release_build(ui);
            //});

            egui::Window::new("Login1").show(ctx, |ui| {
                egui::Grid::new("login_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        let user_prompt = ui.add(egui::Label::new("Username:"));
                        user_prompt.on_hover_text("Your Username...");
                        let _user_prompt2 = ui.add(
                            egui::TextEdit::singleline(username)
                                .hint_text("Your Username to login."),
                        );
                        ui.end_row();

                        let password_prompt = ui.add(egui::Label::new("Password:"));
                        password_prompt.on_hover_text("Your Password...");
                        let _password_prompt2 = ui.add(
                            egui::TextEdit::singleline(password)
                                .password(true)
                                .hint_text("?????"),
                        );
                        ui.end_row();

                        let login_button = ui.add(egui::Button::new("Login"));
                        if login_button.clicked() {
                            if password == "password" {
                                *logged_in = true;
                            } else {
                                *password = "".to_string();
                            }
                        }
                    });
            });
        }
    }
}
