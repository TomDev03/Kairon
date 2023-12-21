#[path = "window_handler/window_handler.rs"] mod window_handler;

fn main() {
    
    pollster::block_on(window_handler::run());

}
