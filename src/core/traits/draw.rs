pub fn draw(value: &impl Drawable) {
    value.draw();
}

pub trait Drawable {
    fn draw(&self);
}
