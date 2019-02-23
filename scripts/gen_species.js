const fs = require('fs');

const asArr = __dirname.split("/")
asArr.pop()
function createPath(list){
	const newPath = asArr.map(v=>v)
	list.forEach(segment => {
		newPath.push(segment)
	});
	return newPath.join("/")
}
const generatedFolder = createPath(["src","generated"])
const generatedJSFolder = createPath(["static","js","generated"])
const speciesFolder = createPath(["species"])

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
const makeBasicHeader = () => {
	return `use crate::generated::species_types::SpeciesTypes;
	use stdweb::Value;
	use stdweb::unstable::TryInto;`
}

genSpeciesEnum = {
	list: [],

	add(item){
		this.list.push(item)
	},
	doGen(){
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
		generated.addFile("species_types", typeStr)
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
		generated.addFile("create_type",createTypeStr)
	}
}
genSpeciesNameGen = {
	list : [],
	add(type,folder) {
		this.list.push( {type,folder})
	},
	geneateName() {
		const names = this.list.map(v=>({
			list : fs.readFileSync(v.folder+"/names","utf8").split("\n"),
			type : v.type
		}))

		JSList.add("names",names)
		const genStr = makeBasicListPicker("names","name")
		generated.addFile("create_name",genStr);

	}
}
genSpeciesDescsGen = {
	list : [],
	add(type,folder) {
		this.list.push( {type,folder})
	},
	doGen() {
		const descs = this.list.map(v=>({
			list : fs.readFileSync(v.folder+"/descriptions","utf8").split("\n"),
			type : v.type
		}))

		JSList.add("descriptions",descs)
		const genStr = makeBasicListPicker("descriptions","description")
		generated.addFile("create_description",genStr);

	}
}
genPossibleImages = {
	list :[],
	add(type,folder) {
		const files = fs.readdirSync(folder+"/images")
		let basePath = createPath(["static","assets","images","species"])
		try{
			fs.mkdirSync(basePath)
		}
		catch(e){
			if(e.code=="EEXIST"){
				console.log(e.path,"already exists (skip creation)")
			} else {
				throw(e)
			}
		}
		const fileList = []
		files.filter(v=>v!=".gitkeep").map(v=>({full:folder+"/images/"+v,name:v})).forEach(v=>{
			const newPath = basePath+"/"+type+"/"
			try{
				fs.mkdirSync(newPath)
			}
			catch(e){
				if(e.code=="EEXIST"){
					console.log(e.path,"already exists (skip creation)")
				} else {
					throw(e)
				}
			}
			const newPathAndName = newPath+v.name
			fs.copyFileSync(v.full,newPathAndName)
			fileList.push("/assets/images/species/"+type+"/"+v.name)
		})
		if(fileList.length == 0) {
			throw(new Error("No images found."))
		}
		this.list.push({type,list:fileList})
	},
	doGen(){
		JSList.add("images",this.list)
		const str = makeBasicListPicker("images","image")
		generated.addFile("genereate_image",str);
	}
}
JSList = {
	list:[],
	types:[],
	addType(type){
		this.types.push(type)
	},
	add(name,list) {
		this.list.push({name,list})
	},
	genGetFromList() {
		const mappedTypes = {}
		const str=`
use crate::generated::species_types::SpeciesTypes;
pub fn type_to_num(species_type : SpeciesTypes) -> i32 {
	match species_type {
		${this.types.map((v,k)=>{mappedTypes[v]=k;return "SpeciesTypes::"+v+"=>{"+k+"}"}).join(",")}
	}
}
`
		generated.addFile("type_to_num",str)
		const lists = {}
		this.list.forEach(v=>{
			const asArr = {}
			v.list.forEach((v)=>{
				asArr[mappedTypes[v.type]] = v.list.map(v=>v)
			})
			lists[v.name] = asArr;
		})
		jsFileStr = `lists = ${JSON.stringify(lists)}`
		generated.addJSFile("lists",jsFileStr)
	}
}
generated = {
	toMod : [],
	addFile(fileName,text) {
		let path = generatedFolder+"/"+fileName+".rs"
		this.toMod.push(fileName)
		fs.writeFileSync(path,text)
	},
	writeMod() {
		str = this.toMod.map(v=>"pub mod "+v+";").join("\n")
		fs.writeFileSync(generatedFolder+"/mod.rs",str)
	},
	addJSFile(fileName,text){
		let path = generatedJSFolder + "/" + fileName+".js"
		fs.writeFileSync(path,text)
	}
}

fs.readdir(speciesFolder,(err,items)=>{
	if(err){
		throw(err);
	}
	items.forEach(v=> {
		const upperCase = v.charAt(0).toUpperCase() + v.slice(1);
		const folder = createPath(["species",v])
		genSpeciesEnum.add(upperCase)
		genSpeciesNameGen.add(upperCase,folder)
		genPossibleImages.add(upperCase,folder)
		genSpeciesDescsGen.add(upperCase,folder)
		JSList.addType(upperCase)
		return {name : v, folder : folder }
	})
	genSpeciesNameGen.geneateName()
	genSpeciesDescsGen.doGen()
	genSpeciesEnum.doGen()
	genPossibleImages.doGen()
	JSList.genGetFromList()
	generated.writeMod()
})
