use cli::{args::Args, run_app};

fn main() {
    let a = Args::init();
    let mut app = run_app(a);
    app.show_header();
    app.print_packet(2);
    app.print_packet(3);
}
