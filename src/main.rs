mod maze_gen;
mod player_movement;
use eframe::{egui, epi};
//use egui::Painter;
use maze_gen::Maze;
use player_movement::Player;
use std::default::Default;

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
            let cell_size = 20.0; // Define the cell_size variable
            let player_pos = self.player.position; // Assuming `position` is a field in `Player`
            let player_rect = egui::Rect::from_min_size(
                egui::Pos2::new(
                    player_pos.x as f32 * cell_size,
                    player_pos.y as f32 * cell_size,
                ),
                egui::Vec2::new(cell_size, cell_size),
            );
            let red = egui::Color32::RED;
            painter.rect_filled(player_rect, 0.0, red);
            // Handle keyboard input
            let mut new_position = self.player.position;
            let mut direction = egui::Vec2::new(0.0, 0.0);
            if ctx.input().key_pressed(egui::Key::ArrowUp) {
                new_position.y -= 1.0;
                direction.y = -1.0;
            }
            if ctx.input().key_pressed(egui::Key::ArrowDown) {
                new_position.y += 1.0;
                direction.y = 1.0;
            }
            if ctx.input().key_pressed(egui::Key::ArrowLeft) {
                new_position.x -= 1.0;
                direction.x = -1.0;
            }
            if ctx.input().key_pressed(egui::Key::ArrowRight) {
                new_position.x += 1.0;
                direction.x = 1.0;
            }

            // Check if the new position is a wall
            if !self.maze.is_wall(self.player.position, direction) {
                // If not, move the player
                self.player.position = new_position;
            }
        });
    }
}

fn main() {
    let start_position = egui::Pos2::new(0.0, 0.0);
    let mut app = MyApp {
        maze: Maze::new(30, 30),
        player: Player::new(start_position), // Add the player field with a default value
    };
    app.maze.generate();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
