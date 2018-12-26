use std::time::Duration;
use yew::prelude::worker::*;
use yew::services::Task;
use yew::services::interval::IntervalService;
use std::collections::HashSet;

pub struct Worker {
    link: AgentLink<Worker>,
    interval: IntervalService,
    task: Box<Task>,
    update_count : i64,
    component_list : HashSet<HandlerId>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Question(String),
}

impl Transferable for Request { }

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Answer(i64),
}

impl Transferable for Response { }

pub enum Msg {
    Updating,
}

impl Agent for Worker {
    // Available:
    // - `Job` (one per bridge)
    // - `Context` (shared in the same thread)
    // - `Public` (separate thread).
    type Reach = Context; // Spawn only one instance per thread (all components could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let duration = Duration::from_secs(3);
        let callback = link.send_back(|_| Msg::Updating);
        let task = interval.spawn(duration, callback);
        js! {
            console.log("I AM CREATED!")
        };
        Worker {
            link,
            interval,
            task: Box::new(task),
            update_count:0,
            component_list : HashSet::new(),
        }
    }
    fn connected(&mut self , id: HandlerId){
        self.component_list.insert(id);
    }
    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                self.update_count = self.update_count + 1;
                for sub in self.component_list.iter(){
                    self.link.response(*sub, Response::Answer(self.update_count));
                }
                js! {
                    console.log("Awesome!")
                };
            }
        }
     }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Question(_) => {
                js! {
                    console.log("Got something?");
                }
                //self.link.response(who, Response::Answer("That's cool!".into()));
            },
        }
    }
}