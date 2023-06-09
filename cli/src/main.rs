use cli::{args::Args, run_app};

fn main() {
    let a = Args::init();
    let idx = a.index();
    let mut app = run_app(a);
    app.show_header();
    app.print_packet(idx);
}
