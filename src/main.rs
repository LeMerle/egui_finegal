use eframe::epi;

mod maze_gen;
mod player_movement;
mod state_loop;
use crate::maze_gen::{maze_to_polylines, Maze};
use eframe::NativeOptions;
use egui::{CentralPanel, Color32, Frame, Pos2, Shape, Stroke};

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}

#[derive(Default)]
struct App {
    maze: Maze,
}

impl App {
    fn new() -> Self {
        let mut maze = Maze::new(250, 250); // Create a new 10x10 maze
        maze.generate(); // Generate the maze

        Self {
            maze,
            // initialize other fields...
        }
    }
}

impl Default for Maze {
    fn default() -> Self {
        // Implement the default behavior for Maze here
        // Return a Maze instance with default values
        Maze {
            width: 0,
            height: 0,
            cells: Vec::new(),
            paths: Vec::new(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Maze Drawer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            // Convert the maze to polylines
            let polylines = maze_to_polylines(&self.maze);

            // Draw the polylines
            let stroke = Stroke::new(1.0, Color32::WHITE);
            let mut frame = Frame::dark_canvas(ui.style());
            frame.show(ui, |ui| {
                let mut painter = ui.painter();
                for polyline in &polylines {
                    painter.add(Shape::line(polyline.clone(), stroke));
                }
            });
        });
    }
}
