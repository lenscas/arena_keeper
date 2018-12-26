extern crate arena_keeper;
extern crate yew;


use yew::prelude::*;
use yew::services::console::ConsoleService;

use arena_keeper::Model;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
