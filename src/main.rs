extern crate arena_keeper;
extern crate yew;


use yew::prelude::*;
use yew::services::console::ConsoleService;

use arena_keeper::Model;

struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
     let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
