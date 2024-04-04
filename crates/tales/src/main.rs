use window_handler;

fn main() {
    pollster::block_on(window_handler::run());
}
