use cli::{args::Args, run_app};

fn main() {
    let a = Args::init();
    let mut app = run_app(a);
    app.show_header();
    app.print_packet(0);
    app.print_packet(3);
    app.print_packet(4);
    app.print_packet(5);
}
