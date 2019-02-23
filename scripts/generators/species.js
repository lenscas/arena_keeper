const makeBasicHeader = () => {
	return `use crate::generated::species_types::SpeciesTypes;
	use stdweb::Value;
	use stdweb::unstable::TryInto;`
}
const makeBasicListPicker = (listName,funName)=> {
	return `
${makeBasicHeader()}
use crate::generated::type_to_num::type_to_num;
pub fn generate_${funName} (specie : SpeciesTypes) -> String {
	let num = type_to_num(specie);
	let v : Value = js!{
		return getFromList("${listName}",@{num})
	};
	let v : String = v.try_into().expect("Something wend wrong generating ${funName}");
	v
}
`
}
module.exports = apis => {
	const fs = require('fs');
	const speciesFolder = apis.appendFile("..","species")
	const speciesEnum = require("../helpers/genSpeciesEnum")
	const genSpesciesText = require("../helpers/genSpeciesTexts")
	const genPossibleImages = require("../helpers/genPossibleImages")
	const JSList = require("../helpers/JSListGen")
	fs.readdir(speciesFolder,(err,items)=>{
		if(err){
			throw(err);
		}
		items.forEach(v=> {
			const upperCase = v.charAt(0).toUpperCase() + v.slice(1);
			const folder = apis.appendFile("..","species",v)
			speciesEnum.add(upperCase)
			genSpesciesText.name.add(upperCase,folder)
			genPossibleImages.add(apis,upperCase,folder)
			genSpesciesText.description.add(upperCase,folder)
			JSList.addType(upperCase)
			return {name : v, folder : folder }
		})
		genSpesciesText.name.doGen(apis,makeBasicListPicker)
		genSpesciesText.description.doGen(apis,makeBasicListPicker)
		speciesEnum.doGen(apis,makeBasicHeader)
		genPossibleImages.doGen(apis,makeBasicListPicker)
		JSList.genGetFromList(apis)
	})

}