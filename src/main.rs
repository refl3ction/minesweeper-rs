#![allow(dead_code, unused_imports)]

use eframe::egui;
use egui::{Button, Ui};
use rand::{thread_rng, Rng};

mod minesweeper;
use minesweeper::*;
mod board;
mod ui;
use ui::*;

pub type Position = (usize, usize);
struct AppState {}

fn main() {
    // Initialize minesweeper here
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(Minesweeper::default())),
    );
}
