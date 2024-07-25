
use nannou::prelude::*;
use crate::cell::CellGrid;
use crate::cell::Cell;
pub struct Model {
    cell_grid: CellGrid,
    cell_size: f32,
    mouse_pos: Vec2,
    mouse_pressed: bool,
    width: usize,
    height: usize,
}


pub fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .view(view).build()
        .unwrap();
    let width = 800;
    let height = 800;
    let cell_size = 10.0;
    let mut cell_grid = CellGrid::new(width/cell_size as usize, height/cell_size as usize);
    // cell_grid.set(200, 700, Cell::Earth);
    Model {
        cell_grid,
        cell_size,
        mouse_pos: vec2(0.0, 0.0),
        mouse_pressed: false,
        width,
        height,
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for i in 0..model.cell_grid.len() {
        let (x,y) = model.cell_grid.get_cords(i, model.cell_size);
        let cell = model.cell_grid.get(x, y).unwrap();
        let (x, y) = model.cell_grid.get_nannou_cords(x, y, model.cell_size);
        match *cell {
            Cell::Fire => {
                draw.rect()
                .x_y(x, y)
                .w_h(model.cell_size, model.cell_size)
                .color(RED);
            }
            Cell::Water => {
                draw.rect()
                .x_y(x, y)
                .w_h(model.cell_size, model.cell_size)
                .color(BLUE);
            }
            Cell::Earth => {
                draw.rect()
                .x_y(x, y)
                .w_h(model.cell_size, model.cell_size)
                .color(GREEN);
            }
            Cell::Empty => {
                
            }
        }
    }


    draw.to_frame(app, &frame).unwrap();
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_pos = pos;
}

pub fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = true;
    
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = false;
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.mouse_pressed {
        model.cell_grid.place_material(model.mouse_pos, Cell::Earth,model.cell_size,  model.width, model.height);
    }
    model.cell_grid = model.cell_grid.apply_rules();
}