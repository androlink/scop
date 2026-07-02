pub struct App<T: Context> {
    context_handler: Option<T>,
}

impl<T: Context> App<T> {
    pub fn new() -> Result<Self, String> {
        todo!()
    }
}

pub trait Context {
    fn setup(&self);

    fn draw(&self);

    fn redraw(&self) {
        self.draw();
    }
    fn key_pressed(&self, event: ()) {}
    fn key_released(&self, event: ()) {}
    fn key_typed(&self, event: ()) {}
}
