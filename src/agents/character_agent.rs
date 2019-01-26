use crate::classes::character::Character;
use crate::classes::character::CharacterTypes;
use std::collections::HashMap;
use yew::prelude::worker::*;
use std::collections::HashSet;

#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct CharacterId(pub u64);

pub struct Worker {
    link: AgentLink<Worker>,
    component_list : HashSet<HandlerId>,
	subbed_to_char_list : HashSet<HandlerId>,
	subbed_to_single_char : HashMap<CharacterId,HashSet<HandlerId>>,
    subbed_to_single_available_char : HashMap<CharacterId,HashSet<HandlerId>>,
    subbed_to_available_list : HashSet<HandlerId>,
	chosen_characters : HashMap<CharacterId,Character>,
	to_be_chosen : HashMap<CharacterId,Character>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SwitchSubscribedAvailableCharacter(CharacterId,CharacterId),
    SwitchSubscribedCharacter(CharacterId,CharacterId),
    UpdateCharacter(CharacterId,Character),
    BuyCharacter(CharacterId),
    GetCharacter(CharacterId),
    GetDoubleCharacter( (CharacterId,CharacterId)),
    GetAvailableChar(CharacterId),
    GetIdList,
    GetAvailableList,
}

impl Transferable for Request { }

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Answer,
    AnswerIdList(Vec<CharacterId>),
    AnswerSingleChar(Character,CharacterId),
    AnswerDoubleChar((CharacterId,Character),((CharacterId,Character)))
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
        let mut agent = Worker {
            link,
            component_list : HashSet::new(),
			subbed_to_char_list : HashSet::new(),
			subbed_to_single_char : HashMap::new(),
			chosen_characters : HashMap::new(),
			to_be_chosen : HashMap::new(),
            subbed_to_available_list : HashSet::new(),
            subbed_to_single_available_char : HashMap::new()
		};
		agent.to_be_chosen.insert(CharacterId { 0:1}, Character::create_character(CharacterTypes::Human));
		agent.to_be_chosen.insert(CharacterId { 0:2}, Character::create_character(CharacterTypes::Merfolk));
		agent.to_be_chosen.insert(CharacterId { 0:3}, Character::create_character(CharacterTypes::Merfolk));
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
                /*
                let m_sub_list = self.subbed_to_single_available_char.get_mut(&old_char_id);
                if let Some(sub_list) = m_sub_list {
                    sub_list.remove(&who);
                }
                self.subbed_to_single_available_char.entry(new_char_id).or_insert(HashSet::new()).insert(who);
                self.respondWithSingleChar(&who, &new_char_id, &self.to_be_chosen);
                */
            },
            Request::UpdateCharacter(char_id,new_character) => {
                let m_chara = self.chosen_characters.get_mut(&char_id);
                if let Some(chara) = m_chara {
                    *chara = new_character;
                }
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
                    self.link.response(who, Response::AnswerSingleChar(chara.clone(),char_id));
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
                    self.link.response(who, Response::AnswerSingleChar(chara.clone(),char_id));
                    let map = self.subbed_to_single_available_char.entry(char_id).or_insert(HashSet::new());
                    map.insert(who);
                }
            },
            Request::GetDoubleCharacter(char_ids) => {
                let char1_id = char_ids.0;
                let char2_id = char_ids.1;
                if let Some(char1) = self.chosen_characters.get(&char1_id) {
                    if let Some(char2) =  self.chosen_characters.get(&char2_id) {
                        let res_char1 = (char1_id,char1.clone());
                        let res_char2 = (char2_id,char2.clone());
                        self.link.response(who, Response::AnswerDoubleChar(res_char1,res_char2));
                        self.subbed_to_single_char.entry(char1_id).or_default().insert(who);
                        self.subbed_to_single_char.entry(char2_id).or_default().insert(who);
                    }
                }
            }

        }
    }
    fn name_of_resource() -> &'static str { "bin/native_worker.js" }
}
impl Worker {
    fn switch_subscibed_char(&mut self, sub : &HandlerId, old_char_id : &CharacterId, new_char_id : &CharacterId,use_available : bool) {
        let sub_list : &mut HashMap<CharacterId,HashSet<HandlerId>>;
        let char_list : & HashMap<CharacterId,Character>;
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
    fn respond_with_single_char(&self, sub :&HandlerId, char_id : &CharacterId, id_list : &HashMap<CharacterId,Character>) {
        let m_chara = id_list.get(char_id);
        if let Some(chara) = m_chara {
            self.link.response(sub.to_owned(), Response::AnswerSingleChar(chara.clone(),*char_id));
        }
    }
    fn send_list(&self,sub : &HandlerId, id_list : &HashMap<CharacterId,Character>){
        let as_vec = id_list
            .iter()
            .map( |v| v.0.to_owned() )
            .collect::<Vec<CharacterId>>();
        self.link.response(sub.to_owned(), Response::AnswerIdList(as_vec));
    }
    fn update_char_list(&self, sub_list : &HashSet<HandlerId>, id_list : &HashMap<CharacterId,Character>){
        for sub in sub_list.iter() {
            self.send_list(sub, id_list);
        }
    }
}