use rand::Rng;

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub visited: bool, // Add the visited field
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<Cell>>,
    pub paths: Vec<Vec<bool>>, // Add this line
}

impl Maze {
    fn default() -> Self {
        Self::new(10, 10) // Default to a 10x10 maze
    }
    pub fn new(width: i32, height: i32) -> Self {
        let cells = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Cell {
                        x,
                        y,
                        visited: false,
                    })
                    .collect()
            })
            .collect();
        let paths = vec![vec![false; width as usize]; height as usize];

        Self {
            width,
            height,
            cells,
            paths,
        }
    }
    fn visited(&self, cell: Cell) -> bool {
        self.cells[cell.y as usize][cell.x as usize].visited
    }

    fn carve_path(&mut self, from: Cell, to: Cell) {
        self.paths[from.y as usize][from.x as usize] = true;
        self.paths[to.y as usize][to.x as usize] = true;
        self.cells[to.y as usize][to.x as usize].visited = true;
    }
    pub fn generate(&mut self) {
        // Start at a random cell
        let mut rng = rand::thread_rng();

        let start_x = if self.width > 0 {
            rng.gen_range(0..self.width)
        } else {
            0
        };

        let start_y = if self.height > 0 {
            rng.gen_range(0..self.height)
        } else {
            0
        };

        // Initialize the stack of cells to visit with the starting cell
        let mut to_visit = vec![self.cells[start_y as usize][start_x as usize]];

        while let Some(cell) = to_visit.pop() {
            // Get the neighbors of the current cell
            let neighbors = self.get_neighbors(cell);

            // If the neighbor hasn't been visited yet, carve a path to it and add it to the stack
            for neighbor in neighbors {
                if !self.visited(neighbor) {}
            }
        }
    }

    fn get_neighbors(&self, cell: Cell) -> Vec<Cell> {
        let mut neighbors = Vec::new();

        if cell.y > 0 {
            neighbors.push(self.cells[(cell.y - 1) as usize][cell.x as usize]);
        }
        if cell.y < self.height - 1 {
            neighbors.push(self.cells[(cell.y + 1) as usize][cell.x as usize]);
        }
        if cell.x > 0 {
            neighbors.push(self.cells[cell.y as usize][(cell.x - 1) as usize]);
        }
        if cell.x < self.width - 1 {
            neighbors.push(self.cells[cell.y as usize][(cell.x + 1) as usize]);
        }

        neighbors
    }
}

pub fn maze_to_polylines(maze: &Maze) -> Vec<Vec<egui::Pos2>> {
    let mut polylines = Vec::new();

    // Create a polyline for each row
    for (y, row) in maze.cells.iter().enumerate() {
        let mut polyline = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            if cell.visited {
                polyline.push(egui::Pos2::new(x as f32, y as f32));
            }
        }
        polylines.push(polyline);
    }

    // Create a polyline for each column
    for x in 0..maze.width {
        let mut polyline = Vec::new();
        for y in 0..maze.height {
            if maze.cells[y as usize][x as usize].visited {
                polyline.push(egui::Pos2::new(x as f32, y as f32));
            }
        }
        polylines.push(polyline);
    }

    polylines
}
