use active_win_pos_rs::{self, get_active_window};
use eframe::egui::{self, mutex::Mutex};
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::{
    sync::{self, atomic::AtomicBool, Arc},
    time::Instant,
};

pub fn clicker(is_running: Arc<AtomicBool>, is_busy: Arc<AtomicBool>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";
    let mut rng = rand::rng();
    loop {
        if is_running.load(sync::atomic::Ordering::Relaxed) {
            if !is_busy.load(sync::atomic::Ordering::Relaxed) {
                if let Ok(active_window) = get_active_window() {
                    let title = active_window.title.to_lowercase();
                    if title.contains(target_window_name) {
                        let _ = enigo.button(Button::Left, Click);
                        let random_delay = rng.random_range(600..900);

                        thread::sleep(Duration::from_millis(random_delay));
                    } else {
                        thread::sleep(Duration::from_millis(1000));
                    }
                } else {
                    thread::sleep(Duration::from_millis(200));
                }
            } else {
                thread::sleep(Duration::from_millis(500));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
pub fn luck(is_luck: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, potion_key: Arc<Mutex<String>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";

    let mut last_potion_time = Instant::now() - Duration::from_secs(300);
    loop {
        if is_luck.load(sync::atomic::Ordering::Relaxed) {
            if let Ok(active_window) = get_active_window() {
                let title = active_window.title.to_lowercase();
                if title.contains(target_window_name) {
                    if last_potion_time.elapsed() >= Duration::from_secs(300) {
                        if is_busy
                            .compare_exchange(
                                false,
                                true,
                                sync::atomic::Ordering::Acquire,
                                sync::atomic::Ordering::Relaxed,
                            )
                            .is_ok()
                        {
                            let _ = enigo.key(
                                Key::Unicode(potion_key.lock().chars().next().unwrap_or('3')),
                                Click,
                            );
                            thread::sleep(Duration::from_millis(100));
                            let _ = enigo.button(Button::Left, Press);
                            thread::sleep(Duration::from_millis(2500));
                            let _ = enigo.button(Button::Left, Release);
                            let _ = enigo.key(Key::Unicode('1'), Click);

                            last_potion_time = Instant::now();

                            is_busy.store(false, sync::atomic::Ordering::Relaxed);
                        } else {
                            thread::sleep(Duration::from_millis(100));
                        }
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                } else {
                    thread::sleep(Duration::from_millis(1000));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
pub fn sell(is_sell: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, time_key: Arc<Mutex<u8>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";

    let mut last_sell_time = Instant::now() - Duration::from_mins(*time_key.lock() as u64);
    loop {
        if is_sell.load(sync::atomic::Ordering::Relaxed) {
            if let Ok(active_win) = get_active_window() {
                let title = active_win.title.to_lowercase();
                if title.contains(target_window_name) {
                    if last_sell_time.elapsed() >= Duration::from_mins(*time_key.lock() as u64) {
                        if is_busy
                            .compare_exchange(
                                false,
                                true,
                                sync::atomic::Ordering::Acquire,
                                sync::atomic::Ordering::Relaxed,
                            )
                            .is_ok()
                        {
                            let _ = enigo.key(Key::Unicode('t'), Click);
                            thread::sleep(Duration::from_millis(600));
                            let _ = enigo.move_mouse(750, 1050, Coordinate::Abs);
                            let _ = enigo.button(Button::Left, Click);
                            thread::sleep(Duration::from_millis(50));
                            let _ = enigo.move_mouse(1750, 480, Coordinate::Abs);
                            let _ = enigo.button(Button::Left, Click);
                            thread::sleep(Duration::from_millis(300));
                            let _ = enigo.move_mouse(1270, 920, Coordinate::Abs);
                            let _ = enigo.button(Button::Left, Click);
                            thread::sleep(Duration::from_millis(50));
                            let _ = enigo.move_mouse(1250, 990, Coordinate::Abs);
                            let _ = enigo.button(Button::Left, Click);
                            //code selling
                            thread::sleep(Duration::from_millis(300));
                            let _ = enigo.key(Key::Unicode('t'), Click);
                            thread::sleep(Duration::from_millis(300));
                            let _ = enigo.key(Key::Unicode('1'), Click);
                            last_sell_time = Instant::now();
                            is_busy.store(false, sync::atomic::Ordering::Relaxed);
                        } else {
                            thread::sleep(Duration::from_millis(100));
                        }
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                } else {
                    thread::sleep(Duration::from_millis(1000));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
