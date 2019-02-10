use yew::prelude::worker::*;
use std::collections::HashSet;
use crate::generated::routes::Windows;

pub struct Worker {
    link: AgentLink<Worker>,
    component_list : HashSet<HandlerId>,
}

#[derive(Copy,Clone, Serialize, Deserialize, Debug,PartialEq, Eq)]
pub enum Request {
    ActivateWindow(Windows),
    DeactivateWindow(Windows)
}

impl Transferable for Request { }

pub enum Msg {}

impl Agent for Worker {
    // Available:
    // - `Job` (one per bridge)
    // - `Context` (shared in the same thread)
    // - `Public` (separate thread).
    type Reach = Context; // Spawn only one instance per thread (all components could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Request;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        Worker {
            link,
            component_list : HashSet::new(),
		}
    }
    fn connected(&mut self , id: HandlerId){
        self.component_list.insert(id);
    }
    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) {
        match msg {
        }
     }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input,_: HandlerId) {
        self.send_message(msg);
    }
}
impl Worker {
    fn send_message(&mut self, res_type :Request) {
        for v in self.component_list.iter() {
            self.link.response(*v,res_type);
        }
    }
}