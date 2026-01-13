use macroquad::prelude::*;

const WIDTH: f32 = 900.;
const HEIGHT: f32 = 600.;

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
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
            cells: vec![WHITE; (width / cell_size) * (height / cell_size)].into_boxed_slice(),
        }
    }

    fn iter_with_coords(&self) -> impl Iterator<Item = (Coords, Color)> {
        self.cells.iter().enumerate().map(|(i, c)| {
            (
                Coords {
                    x: i % self.width,
                    y: i / self.width,
                },
                *c,
            )
        })
    }

    fn draw(&self) {
        clear_background(color_u8!(0x18, 0x18, 0x18, 0xFF));

        self.iter_with_coords()
            .enumerate()
            .for_each(|(i, (Coords { x, y }, color))| {
                // .for_each(|(Coords { x, y }, color)| {
                // SAFETY: Here we work with small number, there will be no loss of precision
                #[allow(clippy::cast_precision_loss)]
                draw_rectangle(
                    (x * self.cell_size) as f32,
                    (y * self.cell_size) as f32,
                    self.cell_size as f32,
                    self.cell_size as f32,
                    color_u8!(i as u8, 0, 0, 255),
                );
            });
            println!("num of cells: {}", self.cells.len());
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    // SAFETY: The number of pixels on the screen size:
    // - Will be positive
    // - Will fit in a usize (at least with current monitors)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let mut map = Map::new(WIDTH as usize, HEIGHT as usize, 10);

    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        map.draw();
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
