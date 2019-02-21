use crate::classes::character::Character;

use std::collections::HashMap;
use indexmap::IndexMap;
use yew::prelude::worker::*;
use std::collections::HashSet;

use crate::agents::clock_agent;

#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct CharacterId(pub u64);

#[derive(Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct CharWithId {
    pub id : CharacterId,
    pub character : Character
}
impl CharWithId {
    pub fn create(id : CharacterId, character : Character) -> CharWithId {
        CharWithId {
            id,
            character
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct MaybeCharWithId {
    pub id : CharacterId,
    pub character : Option<Character>
}
impl MaybeCharWithId {
    pub fn create(id: CharacterId, character : Character) -> MaybeCharWithId {
        MaybeCharWithId {
            id,
            character : Some(character)
        }
    }
    pub fn create_from_ref(id: CharacterId, character : &Character) -> MaybeCharWithId {
        Self::create(id, character.clone())
    }
    pub fn create_from_maybe(id:CharacterId, character: Option<Character>) -> MaybeCharWithId {
        MaybeCharWithId {
            id,
            character
        }
    }
}

pub struct Worker {
    link: AgentLink<Worker>,
    component_list : HashSet<HandlerId>,
	subbed_to_char_list : HashSet<HandlerId>,
	subbed_to_single_char :HashMap<CharacterId,HashSet<HandlerId>>,
    subbed_to_single_available_char : HashMap<CharacterId,HashSet<HandlerId>>,
    subbed_to_available_list : HashSet<HandlerId>,
	chosen_characters : IndexMap<CharacterId,Character>,
	to_be_chosen : IndexMap<CharacterId,Character>,
    _clock_worker: Box<Bridge<clock_agent::Worker>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SetCharacterAsFighting(CharacterId),
    SetCharacterAsNotFighting(CharacterId),
    SwitchSubscribedAvailableCharacter(CharacterId,CharacterId),
    SwitchSubscribedCharacter(CharacterId,CharacterId),
    UpdateCharacter(MaybeCharWithId),
    UpdateMultipleCharacters(Vec<MaybeCharWithId>),
    BuyCharacter(CharacterId),
    GetCharacter(CharacterId),
    GetMultipleCharacters(Vec<CharacterId>),
    GetAvailableChar(CharacterId),
    GetIdList,
    GetAvailableList,
}

impl Transferable for Request { }

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Answer,
    AnswerIdList(Vec<CharacterId>),
    AnswerSingleChar(MaybeCharWithId),
    AnswerMultipleChars(Vec<MaybeCharWithId>)
}

impl Transferable for Response { }

pub enum Msg {
    Updating,
    Tick
}

impl Agent for Worker {
    type Reach = Context; // Spawn only one instance per thread (all components could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        let clock_agent_callback = link.send_back(|_| Msg::Tick);
		let clock_worker = clock_agent::Worker::bridge(clock_agent_callback);

        let mut agent = Worker {
            link,
            component_list : HashSet::new(),
			subbed_to_char_list : HashSet::new(),
			subbed_to_single_char : HashMap::new(),
			chosen_characters : IndexMap::new(),
			to_be_chosen : IndexMap::new(),
            subbed_to_available_list : HashSet::new(),
            subbed_to_single_available_char : HashMap::new(),
            _clock_worker : clock_worker,
		};
		agent.to_be_chosen.insert(CharacterId { 0:1}, Character::create_character());
		agent.to_be_chosen.insert(CharacterId { 0:2}, Character::create_character());
		agent.to_be_chosen.insert(CharacterId { 0:3}, Character::create_character());
        agent
    }
    fn connected(&mut self , id: HandlerId){
        self.component_list.insert(id);
    }
    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                for sub in self.component_list.iter(){
                    self.link.response(*sub, Response::Answer);
                }
            },
            Msg::Tick=> {
                let mut to_update : Vec<CharacterId> = vec!();
                for v in self.chosen_characters.iter_mut() {
                    if v.1.update() {
                        to_update.push(*v.0);
                    }
                }
                for v in to_update.iter() {
                    self.update_char(v);
                }
            }
        }
     }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::SwitchSubscribedCharacter(old_char_id,new_char_id) => {
                self.switch_subscibed_char(&who, &old_char_id, &new_char_id,false);
            },
            Request::SwitchSubscribedAvailableCharacter(old_char_id,new_char_id) => {
                self.switch_subscibed_char(&who, &old_char_id, &new_char_id,true);
            },
            Request::UpdateMultipleCharacters(multiple_characters) => {
                for character in multiple_characters {
                    self.handle(Request::UpdateCharacter(character), who);
                }
            },
            Request::UpdateCharacter(character) => {
                if let Some(new_char) = character.character {
                    if new_char.cur_health <= 0 {
                        self.chosen_characters.remove(&character.id);
                        self.update_char_list(&self.subbed_to_char_list, &self.chosen_characters);
                    } else {
                        self.chosen_characters.insert(character.id, new_char);
                    }
                } else {
                    self.chosen_characters.remove(&character.id);
                    self.update_char_list(&self.subbed_to_char_list, &self.chosen_characters);
                }
                self.update_char(&character.id);
            },
            Request::BuyCharacter(id) => {
                let m_chara = self.to_be_chosen.remove(&id);
                if let Some(chara) = m_chara {
                    self.chosen_characters.insert(id, chara);
                    self.update_char_list(&self.subbed_to_available_list, &self.to_be_chosen);
                    self.update_char_list(&self.subbed_to_char_list, &self.chosen_characters);
                }
            },
            Request::GetIdList => {
                let as_vec = self.chosen_characters
                    .iter()
                    .map( |v| v.0.to_owned() )
                    .collect::<Vec<CharacterId>>();
                self.link.response(who, Response::AnswerIdList(as_vec));
                self.subbed_to_char_list.insert(who);
            },
            Request::GetCharacter(char_id) => {
                let m_chara = self.chosen_characters.get(&char_id);
                if let Some(chara) = m_chara {
                    self.link.response(who, Response::AnswerSingleChar(MaybeCharWithId::create_from_ref(char_id, chara)));
                    let map = self.subbed_to_single_char.entry(char_id).or_insert(HashSet::new());
                    map.insert(who);
                }
            },
            Request::GetAvailableList => {
                let as_vec = self.to_be_chosen
                    .iter()
                    .map( |v| v.0.to_owned() )
                    .collect::<Vec<CharacterId>>();
                self.link.response(who, Response::AnswerIdList(as_vec));
                self.subbed_to_available_list.insert(who);
            },
            Request::GetAvailableChar(char_id) => {
                let m_chara = self.to_be_chosen.get(&char_id);
                if let Some(chara) = m_chara {
                    self.link.response(who, Response::AnswerSingleChar(MaybeCharWithId::create_from_ref(char_id, chara)));
                    let map = self.subbed_to_single_available_char.entry(char_id).or_insert(HashSet::new());
                    map.insert(who);
                }
            },
            Request::GetMultipleCharacters(char_ids) => {
                let chars : Vec<MaybeCharWithId> = char_ids
                    .iter()
                    .map(
                        |v|
                        self.get_char_and_sub(v, who)
                    ).collect();
                self.link.response(who, Response::AnswerMultipleChars(chars));
            },
            Request::SetCharacterAsFighting(char_id) => {
                if let Some(char1) = self.chosen_characters.get_mut(&char_id) {
                    char1.is_fighting = true;
                }
            },
            Request::SetCharacterAsNotFighting(char_id) => {
                if let Some(char1) = self.chosen_characters.get_mut(&char_id) {
                    char1.is_fighting = false;
                }
            }

        }
    }
    fn name_of_resource() -> &'static str { "bin/native_worker.js" }
}
impl Worker {
    fn switch_subscibed_char(&mut self, sub : &HandlerId, old_char_id : &CharacterId, new_char_id : &CharacterId,use_available : bool) {
        let sub_list : &mut HashMap<CharacterId,HashSet<HandlerId>>;
        let char_list : & IndexMap<CharacterId,Character>;
        if use_available {
            sub_list = &mut self.subbed_to_single_available_char;
            char_list = &self.to_be_chosen;
        } else {
            sub_list = &mut self.subbed_to_single_char;
            char_list = &self.chosen_characters;
        }
        let m_sub_list = sub_list.get_mut(old_char_id);
        if let Some(sub_list) = m_sub_list {
            sub_list.remove(&sub);
        }
        sub_list.entry(new_char_id.to_owned()).or_insert(HashSet::new()).insert(sub.to_owned());
        self.respond_with_single_char(sub, &new_char_id, &char_list);
    }
    fn respond_with_single_char(&self, sub :&HandlerId, char_id : &CharacterId, id_list : &IndexMap<CharacterId,Character>) {
        let m_chara = id_list.get(char_id);
        self.link.response(sub.to_owned(), Response::AnswerSingleChar( MaybeCharWithId::create_from_maybe(*char_id, m_chara.cloned())));
    }
    fn send_list(&self,sub : &HandlerId, id_list : &IndexMap<CharacterId,Character>){
        let as_vec = id_list
            .iter()
            .map( |v| v.0.to_owned() )
            .collect::<Vec<CharacterId>>();
        self.link.response(sub.to_owned(), Response::AnswerIdList(as_vec));
    }
    fn update_char_list(&self, sub_list : &HashSet<HandlerId>, id_list : &IndexMap<CharacterId,Character>){
        for sub in sub_list.iter() {
            self.send_list(sub, id_list);
        }
    }
    fn update_char (&self, char_id : &CharacterId){
        if let Some(subs) = self.subbed_to_single_char.get(char_id) {
            for sub in subs.iter() {
                self.respond_with_single_char(sub, char_id, &self.chosen_characters);
            }
        }
    }
    fn get_char_and_sub(&mut self, char_id : &CharacterId , who: HandlerId) -> MaybeCharWithId {
        self.subbed_to_single_char.entry(*char_id).or_default().insert(who);
        MaybeCharWithId::create_from_maybe(
            *char_id,
            self.chosen_characters.get(char_id).cloned()
        )
    }
}