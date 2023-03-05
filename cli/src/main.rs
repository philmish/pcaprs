use cli::{args::Args, run_app};

fn main() {
    let a = Args::init();
    let app = run_app(a);
    app.show_header();
}
