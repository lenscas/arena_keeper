const fs = require("fs")
module.exports =  {
	list: [],

	add(item){
		this.list.push(item)
	},
	doGen(apis,makeBasicHeader){
		let typeStr = `
use core::fmt;
#[derive(Eq,Hash,PartialEq,Copy,Clone,Debug,Serialize,Deserialize)]
pub enum SpeciesTypes {`

		typeStr += this.list.join(",")
		typeStr += `
}
impl fmt::Display for SpeciesTypes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}
`
		apis.writeToMod(apis.generatedFolder,"species_types")
		fs.writeFileSync(apis.generatedFolder+"/species_types.rs",typeStr)
		let createTypeStr = `
${makeBasicHeader()}
pub fn generate_type() -> SpeciesTypes {
	let v: Value = js!{
		return getMax(${this.list.length})
	};
	let v : i64 = v.try_into().expect("Something wend wrong");
	match v {
`
		createTypeStr +=this.list.map( (v,k) =>
			k +" => { SpeciesTypes::"+v+"}"
		).join(",")
		createTypeStr+="_=>{unreachable!()}}}"
		apis.writeToMod(apis.generatedFolder,"generate_type")
		fs.writeFileSync(apis.generatedFolder+"/generate_type.rs",createTypeStr)
	}
}