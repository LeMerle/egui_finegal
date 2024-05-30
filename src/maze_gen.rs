use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Cell {
    pub visited: bool,
    pub walls: [bool; 4], // top, right, bottom, left
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            visited: self.visited,
            walls: self.walls,
        }
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![
            vec![
                Cell {
                    visited: false,
                    walls: [true; 4],
                };
                width
            ];
            height
        ];
        Self { grid }
    }

    pub fn generate(&mut self) {
        let start_x = rand::random::<usize>() % self.grid[0].len();
        let start_y = rand::random::<usize>() % self.grid.len();
        self.visit(start_x, start_y);
    }

    fn visit(&mut self, x: usize, y: usize) {
        let dx = [0, 1, 0, -1];
        let dy = [-1, 0, 1, 0];
        let mut rng = thread_rng();
        let dir_order: Vec<usize> = (0..4)
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, 4)
            .cloned()
            .collect();

        self.grid[y][x].visited = true;

        for &dir in dir_order.iter() {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];

            if nx >= 0
                && nx < self.grid[0].len() as i32
                && ny >= 0
                && ny < self.grid.len() as i32
                && !self.grid[ny as usize][nx as usize].visited
            {
                self.grid[y][x].walls[dir] = false;
                self.grid[ny as usize][nx as usize].walls[(dir + 2) % 4] = false;
                self.visit(nx as usize, ny as usize);
            }
        }
    }
    pub fn get_start_position(&self) -> egui::Pos2 {
        // Replace this with your actual logic
        egui::Pos2::new(0.0, 0.0)
    }
    pub fn is_open(&self, position: egui::Pos2, direction: Direction) -> bool {
        let x = position.x as usize;
        let y = position.y as usize;

        match direction {
            Direction::Up => y > 0 && !self.grid[y - 1][x].walls[2],
            Direction::Right => x < self.grid[0].len() - 1 && !self.grid[y][x + 1].walls[3],
            Direction::Down => y < self.grid.len() - 1 && !self.grid[y + 1][x].walls[0],
            Direction::Left => x > 0 && !self.grid[y][x - 1].walls[1],
        }
    }
}
