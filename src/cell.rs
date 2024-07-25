use nannou::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Fire,
    Water,
    Earth,
}

pub struct CellGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl CellGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::Empty; width * height];
        Self {
            cells,
            width,
            height,
        }
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y * self.width + x)
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        match self.get_mut(x, y) {
            Some(c) => *c = cell,
            None => (),
        }
    }
    pub fn get_cords(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;
        (x,y)
    }
    pub fn get_nannou_cords(&self, x: usize, y: usize, cell_size:f32) -> (f32, f32) {
        let x = x as f32 * cell_size - (self.width as f32*cell_size) as f32 /2.0;
        let y = y as f32 * cell_size - (self.height as f32*cell_size) as f32 /2.0;
        (x, y)
    }

    pub fn place_material(&mut self,mouse_pos:Vec2, material: Cell,cell_size: f32, model_width: usize, model_height: usize) {
        if mouse_pos.x <= -(model_width as f32 / 2.0) || mouse_pos.x >= model_width as f32 / 2.0 || mouse_pos.y <= -(model_height as f32 / 2.0) || mouse_pos.y >= model_height as f32 / 2.0 {
            return;
        }
        if self.get(mouse_pos.x as usize, mouse_pos.y as usize) == Some(&Cell::Earth) && (material == Cell::Earth || material == Cell::Water) {
            return;
        }
        if self.get(mouse_pos.x as usize, mouse_pos.y as usize) == Some(&Cell::Water) && (material == Cell::Water || material == Cell::Fire) {
            return;
        }

        let x = (mouse_pos.x+(model_width as f32 / 2.0)) as usize / cell_size as usize;
        let y = (mouse_pos.y+(model_height as f32 / 2.0)) as usize / cell_size as usize;
        self.set(x, y, material);
    }

    pub fn apply_rules(&self) -> CellGrid {
        let mut new_cells = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get(x, y).unwrap();
                match cell {
                    Cell::Empty => {},
                    Cell::Fire => {Cell::Empty;},
                    Cell::Water => {Cell::Water;},
                    Cell::Earth => {
                        if self.get(x, y-1) == Some(&Cell::Empty) {
                            new_cells.set(x, y-1, Cell::Earth);
                            new_cells.set(x, y, Cell::Empty);
                        } else if self.get(x-1, y-1) == Some(&Cell::Empty) && y>0 {
                            new_cells.set(x, y, Cell::Empty);
                            new_cells.set(x-1, y-1, Cell::Earth);
                        }else if self.get(x+1, y-1) == Some(&Cell::Empty) && y>0 {
                            new_cells.set(x, y, Cell::Empty);
                            new_cells.set(x+1, y-1, Cell::Earth);
                        } else {
                            new_cells.set(x, y, Cell::Earth);
                        }
                    },
                };
            }
        }
        new_cells
    }

}
