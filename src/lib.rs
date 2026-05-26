use crate::application::Application;

pub mod application;

pub fn start(){
    Application::new().run();
}