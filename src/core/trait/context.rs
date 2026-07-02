trait Context<T> {
    fn draw(&self);

    fn redraw(&self) {
        self.draw();
    }
    fn key_pressed(&self, event: ()) {}
    fn key_released(&self, event: ()) {}
    fn key_typed(&self, event: ()) {}
}
