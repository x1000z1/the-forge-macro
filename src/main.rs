#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread,
};

use active_win_pos_rs::get_active_window;
use eframe::egui::{self, mutex::Mutex};

mod fonts;
mod macro_forge;
mod switch_ui;

fn main() {
    let icon = fonts::load_icon();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 250.0])
            .with_resizable(false)
            .with_always_on_top()
            .with_decorations(true)
            .with_transparent(true)
            .with_icon(icon),

        ..Default::default()
    };
    let is_clicked = Arc::new(AtomicBool::new(false));
    let is_luck = Arc::new(AtomicBool::new(false));
    let is_sell = Arc::new(AtomicBool::new(false));
    let is_busy = Arc::new(AtomicBool::new(false));

    let potion_key = Arc::new(Mutex::new("3".to_string()));
    let time_key = Arc::new(Mutex::new(15));

    let clicker_busy_flag = is_busy.clone();
    let clicker_running_flag = is_clicked.clone();

    thread::spawn(move || {
        macro_forge::clicker(clicker_running_flag, clicker_busy_flag);
    });

    let luck_busy_flag = is_busy.clone();
    let luck_running_flag = is_luck.clone();
    let luck_key_flag = potion_key.clone();

    thread::spawn(move || {
        macro_forge::luck(luck_running_flag, luck_busy_flag, luck_key_flag);
    });

    let sell_busy_flag = is_busy.clone();
    let sell_running_flag = is_sell.clone();
    let timme_key_flag = time_key.clone();
    thread::spawn(move || {
        macro_forge::sell(sell_running_flag, sell_busy_flag, timme_key_flag);
    });
    eframe::run_native(
        "TFM",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(MyEguiApp::new(
                cc, is_clicked, is_luck, is_sell, potion_key, time_key,
            )))
        }),
    );
}

struct MyEguiApp {
    pub is_clicked: Arc<AtomicBool>,
    pub is_luck: Arc<AtomicBool>,
    pub is_sell: Arc<AtomicBool>,
    pub potion_key: Arc<Mutex<String>>,
    pub time_key: Arc<Mutex<u8>>,
}

impl MyEguiApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        is_clicked: Arc<AtomicBool>,
        is_luck: Arc<AtomicBool>,
        is_sell: Arc<AtomicBool>,
        potion_key: Arc<Mutex<String>>,
        time_key: Arc<Mutex<u8>>,
    ) -> Self {
        let my_font_data = include_bytes!("../assets/Montserrat-SemiBold.ttf");
        let my_fonts = fonts::font_set(my_font_data);
        _cc.egui_ctx.set_fonts(my_fonts);
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            is_clicked,
            is_luck,
            is_sell,
            potion_key,
            time_key,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(window) = get_active_window() {
            if window.title.to_lowercase().contains("roblox") {
                let r_x = window.position.x as f32;
                let r_y = window.position.y as f32;

                let target_pos = egui::pos2(r_x + 8.0, r_y + 32.0);

                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target_pos));
            }
        }
        // Resizable window fn
        // let screen_rect = ctx.input(|i| i.viewport_rect());

        // let current_scale = ctx.pixels_per_point();

        // let physical_width = screen_rect.width() * current_scale;

        // let new_scale = (physical_width / 800.0).clamp(1.0, 1.8);

        // if (new_scale - current_scale).abs() > 0.01 {
        //     ctx.set_pixels_per_point(new_scale);
        // }
        // fn end
        egui::TopBottomPanel::bottom("footer_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(2.0);
                let _ = ui.columns(3, |columns| {
                    columns[1].vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center)
                                    .with_main_align(egui::Align::Center),
                                |ui| {
                                    ui.label(egui::RichText::new("v0.1.0").small().weak());
                                    ui.label(egui::RichText::new("|").small().weak());
                                    ui.hyperlink_to(
                                        egui::RichText::new("by @x1000z1").small().weak(),
                                        "https://github.com/x1000z1",
                                    );
                                },
                            );
                        });
                    });
                });
                ui.add_space(2.0);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading(egui::RichText::new("The Forge Macro").size(28.0).strong());
                    ui.add_space(10.0);
                    ui.label("Advanced automation for The Forge");
                    ui.add_space(15.0);
                });
            });
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width() - 9.0);
                    egui::Grid::new("settings_grid")
                        .num_columns(2)
                        .spacing([15.0, 4.0])
                        .striped(false)
                        .show(ui, |ui| {
                            ui.label("⛏ Mining Clicker");
                            let mut mining_state =
                                self.is_clicked.load(std::sync::atomic::Ordering::Relaxed);
                            ui.horizontal(|ui| {
                                if switch_ui::toggle_ui(ui, &mut mining_state).changed() {
                                    self.is_clicked
                                        .store(mining_state, std::sync::atomic::Ordering::Relaxed);
                                }
                            });
                            ui.end_row();
                            ui.label("🍀 Luck Potion");
                            ui.horizontal(|ui| {
                                let mut luck_state =
                                    self.is_luck.load(std::sync::atomic::Ordering::Relaxed);
                                if switch_ui::toggle_ui(ui, &mut luck_state).changed() {
                                    self.is_luck
                                        .store(luck_state, std::sync::atomic::Ordering::Relaxed);
                                }
                                ui.add_space(10.0);

                                ui.label("Key:");

                                {
                                    let mut key = self.potion_key.lock();
                                    ui.add(
                                        egui::TextEdit::singleline(&mut *key)
                                            .desired_width(25.0)
                                            .char_limit(1)
                                            .horizontal_align(egui::Align::Center),
                                    );
                                }
                            });
                            ui.end_row();
                            ui.label("💰 Auto Sell");
                            let mut sell_state =
                                self.is_sell.load(std::sync::atomic::Ordering::Relaxed);
                            ui.horizontal(|ui| {
                                if switch_ui::toggle_ui(ui, &mut sell_state).changed() {
                                    self.is_sell
                                        .store(sell_state, std::sync::atomic::Ordering::Relaxed);
                                }
                                ui.add_space(10.0);
                                ui.label("Dur:");
                                let mut dur = self.time_key.lock();
                                ui.scope(|ui| {
                                    ui.style_mut().spacing.interact_size.x = 0.0;
                                    ui.style_mut().spacing.item_spacing.x = 0.0;
                                    ui.add_sized(
                                        [32.0, 20.0],
                                        egui::DragValue::new(&mut *dur)
                                            .range(1..=60)
                                            .max_decimals(0)
                                            .speed(0.5),
                                    );
                                });
                            });
                            ui.end_row();
                        });
                });
            });
        });
    }
}
