const JSList = require("./JSListGen")
const fs = require("fs")
module.exports = {
	list :[],
	add(apis, type,folder) {
		const files = fs.readdirSync(folder+"/images")
		let basePath = apis.appendFile("..","static","assets","images","species")
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
	doGen(apis,makeBasicListPicker){
		JSList.add("images",this.list)
		const genStr = makeBasicListPicker("images","image")
		apis.writeToMod(apis.generatedFolder,"generate_image")
		fs.writeFileSync(apis.generatedFolder+"/generate_image.rs",genStr)

	}
}