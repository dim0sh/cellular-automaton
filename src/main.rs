mod app;
mod cell;

fn main() {
    nannou::app(app::model).update(app::update).run();
}
