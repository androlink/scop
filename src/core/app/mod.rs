pub struct App {}

impl App {
    pub fn new() -> Result<Self, String> {
        todo!()
    }
    fn add_plugin<P: Plugin>(mut self, plugin: P) {
        plugin.build(&mut self);
    }

    fn run(&self) {}
}

trait Plugin {
    fn build(&self, app: &mut App);
}

struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, _app: &mut App) {
        println!("test");
    }
}

fn test() {
    App::new().unwrap().add_plugin(TestPlugin);
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
