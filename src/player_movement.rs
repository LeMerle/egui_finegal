use crate::maze_gen::Direction;
use crate::maze_gen::Maze; // Add this line to import the Maze type

pub struct Player {
    pub position: egui::Pos2,
    pub path: Vec<egui::Pos2>,
}

impl Player {
    pub fn default() -> Self {
        Player {
            position: egui::Pos2::new(0.0, 0.0), // replace with your default position
            path: Vec::new(),                    // replace with your default path
        }
    }
    pub fn new(start_position: egui::Pos2) -> Self {
        Self {
            position: start_position,
            path: vec![start_position],
        }
    }
    pub fn handle_input(&mut self, input: &egui::InputState, maze: &Maze) {
        let movement = egui::Vec2::new(1.0, 1.0); // Adjust this based on your maze's cell size

        if input.key_pressed(egui::Key::W) && maze.is_open(self.position, Direction::Up) {
            self.position.y -= movement.y;
        }
        if input.key_pressed(egui::Key::A) && maze.is_open(self.position, Direction::Left) {
            self.position.x -= movement.x;
        }
        if input.key_pressed(egui::Key::S) && maze.is_open(self.position, Direction::Down) {
            self.position.y += movement.y;
        }
        if input.key_pressed(egui::Key::D) && maze.is_open(self.position, Direction::Right) {
            self.position.x += movement.x;
        }

        self.path.push(self.position);
    }

    pub fn draw(&self, painter: &mut egui::Painter) {
        let player_color = egui::Color32::from_rgb(255, 0, 0); // Red
        let path_color = egui::Color32::from_rgb(255, 105, 180); // Pink
        let stroke = egui::Stroke::new(2.0, path_color);

        // Draw the player
        let player_rect = egui::Rect::from_min_size(
            self.position,
            egui::Vec2::new(10.0, 10.0), // Adjust this based on your maze's cell size
        );
        painter.rect_filled(player_rect, 0.0, player_color);

        // Draw the path
        if self.path.len() > 1 {
            for i in 0..self.path.len() - 1 {
                painter.line_segment([self.path[i], self.path[i + 1]], stroke);
            }
        }
    }
}
