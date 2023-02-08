use egui::{Button, Ui};

use crate::minesweeper::{GameDifficult, Minesweeper};

impl eframe::App for Minesweeper {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(&ctx, |ui| {
            ui.heading("Minesweeper");
        });

        // Display logic
        // for minesweeper.board.width
        // for minesweeper.board.height
        // if is open { get count }
        // if is flag { show flag }
        // else show closed
        // if game lost && is mine { show mines }

        // if self.show_board {
        //     egui::CentralPanel::default().show(ctx, |ui| ui::draw_board(ui, &self.board));
        // } else {
        //     egui::CentralPanel::default().show(ctx, |ui| {
        //         ui.heading("Choose the difficult");
        //         if ui.button("Beginner").clicked() {
        //             self.board = Board::new(GameDifficult::BEGINNER);
        //             self.show_board = true
        //         }
        //         if ui.button("Intermediate").clicked() {
        //             self.board = Board::new(GameDifficult::INTERMEDIATE);
        //             self.show_board = true
        //         }
        //     });
        // }

        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // if ui.button("No!").clicked() {
                        //     ctx.
                        // }
                        if ui.button("Yes!").clicked() {
                            self.can_exit = true;
                            frame.quit();
                        }
                    });
                });
        }
    }
}

// pub fn draw_board(ui: &mut Ui, board: &Board) {
//     for i in 0..board.width {
//         // Draw columns
//         ui.horizontal(|ui| {
//             for j in 0..board.length {
//                 let block = Button::new(format!("# {i} {j}"));

//                 let block_handler = ui.add(block);

//                 if block_handler.secondary_clicked() {
//                     println!("right click {i} {j}");
//                 }

//                 if block_handler.clicked() {
//                     println!("left click {i} {j}");
//                 }
//             }
//         });
//     }
// }
