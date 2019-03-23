use crate::funcs::random::get_max;
use std::collections::HashSet;
use yew::prelude::worker::*;

use crate::agents::clock_agent;

pub struct Worker {
	link: AgentLink<Worker>,
	component_list : HashSet<HandlerId>,
	_clock_worker: Box<Bridge<clock_agent::Worker>>,
	blocks : Vec<(f64,f64)>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {}

impl Transferable for Request { }

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
	NewPos( Vec<(f64,f64)> )
}

impl Transferable for Response { }

pub enum Msg {
    Tick
}

impl Agent for Worker {
    type Reach = Context; // Spawn only one instance per thread (all components could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        //*
		let clock_agent_callback = link.send_back(|_| Msg::Tick);
		let clock_worker = clock_agent::Worker::bridge(clock_agent_callback);
        //*/
		Worker {
            link,
			_clock_worker : clock_worker,
			component_list : HashSet::new(),
			blocks : vec![
				(0.0,0.0),
				(2.0,2.0),
				(10.0,2.0),
				(6.0,0.5),
				(2.0,10.0),
				(3.0,8.0),
				(5.0,20.0),
			]
		}
    }
    fn connected(&mut self , id: HandlerId){
        self.component_list.insert(id);
    }
    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) {
		match msg {
			Msg::Tick => {
				let mut as_cloned : Vec<(f64,f64)> = Vec::new();
				self.blocks.iter_mut().for_each(
					|block| {
						block.0 += f64::from(get_max(4) - 1) ;
						block.1 += f64::from(get_max(4) - 1 );
						as_cloned.insert(as_cloned.len(), block.clone());
					}
				);
				self.component_list.iter().for_each(
					|component| self.link.response(
						*component,Response::NewPos(
							as_cloned.clone()
						)
					)
				)
			}
		}
     }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, _: Self::Input, _: HandlerId) {

    }
}