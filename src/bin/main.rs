#![recursion_limit="512"]
extern crate arena_keeper;
extern crate yew;
extern crate web_logger;

use yew::prelude::*;

use arena_keeper::Model;

fn main() {
    web_logger::init();
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
