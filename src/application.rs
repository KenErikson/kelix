use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Application;

impl Application {

    pub fn new() -> Self {
        Application
    }


    pub fn run(&mut self) {
        println!("Kelix is running!");

        enable_raw_mode().unwrap();
    }
}


impl Drop for Application {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

