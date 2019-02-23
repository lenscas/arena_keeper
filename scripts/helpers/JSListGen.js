const fs = require("fs")
module.exports = {
	list:[],
	types:[],
	addType(type){
		this.types.push(type)
	},
	add(name,list) {
		this.list.push({name,list})
	},
	genGetFromList(apis) {
		const mappedTypes = {}
		const genStr=`
use crate::generated::species_types::SpeciesTypes;
pub fn type_to_num(species_type : SpeciesTypes) -> i32 {
	match species_type {
		${this.types.map((v,k)=>{mappedTypes[v]=k;return "SpeciesTypes::"+v+"=>{"+k+"}"}).join(",")}
	}
}
`
		apis.writeToMod(apis.generatedFolder,"type_to_num")
		fs.writeFileSync(apis.generatedFolder+"/type_to_num.rs",genStr)

		const lists = {}
		this.list.forEach(v=>{
			const asArr = {}
			v.list.forEach((v)=>{
				asArr[mappedTypes[v.type]] = v.list.map(v=>v)
			})
			lists[v.name] = asArr;
		})
		jsFileStr = `lists = ${JSON.stringify(lists)}`
		fs.writeFileSync(apis.generatedJSPath+"/lists.js",jsFileStr)
	}
}