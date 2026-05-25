
pub fn start() {
    let mut app = Application::new();
    app.run();
}

pub struct Application;

impl Application {

    pub fn new() -> Self {
        Application
    }


    pub fn run(&mut self) {
        println!("Kelix is running!");
    }
}
