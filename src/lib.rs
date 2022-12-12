use std::collections::HashSet;
use rand::Rng;

#[derive(Default, Clone, Copy)]
pub struct Cell {
    pub is_exposed: bool,
    pub is_flagged: bool,
    pub has_mine: bool,
    pub surrounding_gophers: usize
}

pub enum CellResult {
    Exposed,
    HasMine,
    Win
}

pub struct GopherSweeper {
    pub width: usize,
    pub height: usize,
    pub gophers: usize,
    pub remaining_cells: usize,
    field: Vec<Vec<Cell>>
}

impl GopherSweeper {
    pub fn new(width: usize, height: usize, gophers: usize) -> Self {
        let mut result = GopherSweeper {
            width, height, gophers,
            field: Vec::with_capacity(height),
            remaining_cells: width * height - gophers
        };

        for y in 0..height {
            result.field.push(Vec::with_capacity(width));

            for _ in 0..width {
                result.field[y].push(Cell::default());
            }
        }

        let mut rng = rand::thread_rng();
        let mut random_coords: (usize, usize);
        let mut planted_gophers: HashSet<(usize, usize)> = HashSet::with_capacity(gophers);

        while planted_gophers.len() < gophers {
            random_coords = (rng.gen_range(0..width), rng.gen_range(0..height));

            if planted_gophers.insert(random_coords) {
                result.field[random_coords.1][random_coords.0].has_mine = true;

                for (x, y) in result.surrounding_cell_indexes(random_coords.0, random_coords.1) {
                    result.field[y][x].surrounding_gophers += 1;
                }
            }
        }

        result
    }

    pub fn cell(&self, x: usize, y: usize) -> Cell {
        self.field[y][x]
    }

    pub fn set_flag(&mut self, x: usize, y: usize) {
        let mut cell = &mut self.field[y][x];
        cell.is_flagged = !cell.is_flagged;
    }

    pub fn try_expose_cell(&mut self, x: usize, y: usize) -> CellResult {
        let cell = &self.field[y][x];
        if cell.has_mine { return CellResult::HasMine }
        self.expose_recursively(x, y);
        if self.remaining_cells == 0 { return CellResult::Win }
        CellResult::Exposed
    }

    fn expose_recursively(&mut self, x: usize, y: usize) {
        let mut cell = &mut self.field[y][x];

        cell.is_exposed = true;
        self.remaining_cells -= 1;

        if cell.surrounding_gophers == 0 {
            for (x, y) in self.surrounding_cell_indexes(x, y) {
                if !self.field[y][x].is_exposed {
                    self.expose_recursively(x, y);
                }
            }
        }
    }

    fn surrounding_cell_indexes(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];

        let width = self.width;
        let height = self.height;

        if x > 0 { result.push((x - 1, y)); }
        if y > 0 { result.push((x, y - 1)); }
        if x + 1 < width { result.push((x + 1, y)); }
        if y + 1 < height { result.push((x, y + 1)); }
        if x > 0 && y > 0 { result.push((x - 1, y - 1)); }
        if x > 0 && y + 1 < height { result.push((x - 1, y + 1)); }
        if x + 1 < width && y > 0 { result.push((x + 1, y - 1)); }
        if x + 1 < width && y + 1 < height { result.push((x + 1, y + 1)); }

        result
    }
}

impl<'a> IntoIterator for &'a GopherSweeper {
    type Item = &'a Vec<Cell>;
    type IntoIter = std::slice::Iter<'a, Vec<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.field.iter()
    }
}
