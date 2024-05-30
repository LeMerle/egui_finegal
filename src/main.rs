mod maze_gen;
mod player_movement; // Add the player_movement module here
use eframe::{egui, epi};
//use egui::Painter;
use maze_gen::Maze;
use player_movement::Player; // Import the Player struct from the player_movement module
use std::default::Default; // Import the Default trait

struct MyApp {
    maze: Maze,
    player: Player,
}
impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Maze Renderer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            for (y, row) in self.maze.grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let top_left = egui::pos2(x as f32 * 20.0, y as f32 * 20.0);
                    let bottom_right = egui::pos2((x as f32 + 1.0) * 20.0, (y as f32 + 1.0) * 20.0);
                    let rect = egui::Rect::from_min_max(top_left, bottom_right);

                    let color = if cell.visited {
                        egui::Color32::from_rgb(255, 255, 255)
                    } else {
                        egui::Color32::from_rgb(0, 0, 0)
                    };

                    painter.rect_filled(rect, 0.0, color);

                    // Draw walls
                    let wall_thickness: f32 = 2.0;
                    let wall_color = egui::Color32::from_rgb(0, 0, 255);
                    let stroke = egui::Stroke::new(wall_thickness, wall_color);

                    if cell.walls[0] {
                        // top wall
                        let start = rect.min;
                        let end = egui::Pos2::new(rect.max.x, rect.min.y);
                        painter.line_segment([start, end], stroke);
                    }
                    if cell.walls[1] {
                        // right wall
                        let start = egui::Pos2::new(rect.max.x, rect.min.y);
                        let end = rect.max;
                        painter.line_segment([start, end], stroke);
                    }
                    if cell.walls[2] {
                        // bottom wall
                        let start = egui::Pos2::new(rect.min.x, rect.max.y);
                        let end = rect.max;
                        painter.line_segment([start, end], stroke);
                    }
                    if cell.walls[3] {
                        // left wall
                        let start = rect.min;
                        let end = egui::Pos2::new(rect.min.x, rect.max.y);
                        painter.line_segment([start, end], stroke);
                    }
                }
            }
        });
    }
}

fn main() {
    let mut app = MyApp {
        maze: Maze::new(20, 20),
        player: Player::default(), // Add the player field with a default value
    };
    app.maze.generate();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
