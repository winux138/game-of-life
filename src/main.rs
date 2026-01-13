use macroquad::{
    prelude::*,
    rand::{rand, srand},
};

use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: f32 = 900.;
const HEIGHT: f32 = 600.;
const CELL_SIZE: usize = 10;

// SAFETY: The number of pixels on the screen size:
// - Will be positive
// - Will fit in a usize (at least with current monitors)
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
const CELLS_WIDTH: usize = WIDTH as usize / CELL_SIZE;
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
const CELLS_HEIGHT: usize = HEIGHT as usize / CELL_SIZE;

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    coords: Coords,
    color: Color,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    cell_size: usize,
    /// Cells looks like that:
    /// [
    /// {x = 0, y = 0 }, {x = 1, y = 0 }, {x = 2, y = 0 }, {x = 3, y = 0 },
    /// {x = 0, y = 1 }, {x = 1, y = 1 }, {x = 2, y = 1 }, {x = 3, y = 1 },
    /// {x = 0, y = 2 }, {x = 1, y = 2 }, {x = 2, y = 2 }, {x = 3, y = 2 },
    /// {x = 0, y = 3 }, {x = 1, y = 3 }, {x = 2, y = 3 }, {x = 3, y = 3 }
    /// ]
    cells: Box<[Color]>,
}

impl Map {
    fn new(width: usize, height: usize, cell_size: usize) -> Self {
        Self {
            width,
            height,
            cell_size,
            cells: vec![BLACK; width * height].into_boxed_slice(),
        }
    }

    fn from_coords(width: usize, height: usize, cell_size: usize, coords: &[Coords]) -> Self {
        let mut s = Self::new(width, height, cell_size);
        for coord in coords {
            *s.at_mut_ref(*coord) = WHITE;
        }
        s
    }

    fn iter_cells(&self) -> impl Iterator<Item = Cell> {
        self.cells.iter().enumerate().map(|(i, c)| Cell {
            coords: Coords {
                x: i % self.width,
                y: i / self.width,
            },
            color: *c,
        })
    }

    fn at(&self, Coords { x, y }: Coords) -> Color {
        self.cells[x + y * self.width]
    }

    fn at_mut_ref(&mut self, Coords { x, y }: Coords) -> &mut Color {
        &mut self.cells[x + y * self.width]
    }

    fn get_neighbours(&self, Coords { x, y }: Coords) -> [Color; 8] {
        [
            self.at(Coords {
                x: (x as i32 - 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 - 1).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 - 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 - 0).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 - 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 + 1).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 + 0).rem_euclid(self.width as i32) as usize,
                y: (y as i32 - 1).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 + 0).rem_euclid(self.width as i32) as usize,
                y: (y as i32 + 1).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 + 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 - 1).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 + 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 - 0).rem_euclid(self.height as i32) as usize,
            }),
            self.at(Coords {
                x: (x as i32 + 1).rem_euclid(self.width as i32) as usize,
                y: (y as i32 + 1).rem_euclid(self.height as i32) as usize,
            }),
        ]
    }

    fn next_generation(&mut self) {
        let new_cells = self
            .iter_cells()
            .map(|c| {
                match (
                    c.color,
                    self.get_neighbours(c.coords)
                        .iter()
                        .filter(|e| **e == WHITE)
                        .count(),
                ) {
                    (WHITE, 2 | 3) => WHITE,
                    (BLACK, 3) => WHITE,
                    _ => BLACK,
                }
            })
            .collect();
        self.cells = new_cells;
    }

    fn draw(&self) {
        clear_background(color_u8!(0x18, 0x18, 0x18, 0xFF));

        self.iter_cells().for_each(
            |Cell {
                 coords: Coords { x, y },
                 color,
             }| {
                // SAFETY: Here we work with small number, there will be no loss of precision
                #[allow(clippy::cast_precision_loss)]
                draw_rectangle(
                    (x * self.cell_size) as f32,
                    (y * self.cell_size) as f32,
                    self.cell_size as f32,
                    self.cell_size as f32,
                    color,
                );
            },
        );
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should go forward")
            .as_secs(),
    );

    let alive_cells: Vec<_> = (0..2000)
        .map(|_| Coords {
            x: (rand() as usize) % CELLS_WIDTH,
            y: (rand() as usize) % CELLS_HEIGHT,
        })
        .collect();

    // let alive_cells = vec![
    //     Coords{x: 50, y: 30},
    //     Coords{x: 51, y: 30},
    //     Coords{x: 49, y: 30},
    // ];

    let mut map = Map::from_coords(CELLS_WIDTH, CELLS_HEIGHT, CELL_SIZE, &alive_cells);

    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        if get_keys_down().contains(&KeyCode::Space) {
            map.next_generation();
        }
        map.draw();

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn modulus() {
        let a: i32 = -1;
        assert_ne!(a % 5, 4);
        assert_eq!(a.rem_euclid(5), 4);
    }
}
