const JSList = require("./JSListGen")
const fs = require("fs")
module.exports = {
	name : {
		list : [],
		add(type,folder) {
			this.list.push( {type,folder})
		},
		doGen(apis,makeBasicListPicker) {
			const names = this.list.map(v=>({
				list : fs.readFileSync(v.folder+"/names","utf8").split("\n"),
				type : v.type
			}))

			JSList.add("names",names)
			const genStr = makeBasicListPicker("names","name")
			apis.writeToMod(apis.generatedFolder,"create_name")
			fs.writeFileSync(apis.generatedFolder+"/create_name.rs",genStr)

		}

	},
	description : {
		list : [],
		add(type,folder) {
			this.list.push( {type,folder})
		},
		doGen(apis,makeBasicListPicker) {
			const descs = this.list.map(v=>({
				list : fs.readFileSync(v.folder+"/descriptions","utf8").split("\n"),
				type : v.type
			}))
			JSList.add("descriptions",descs)
			const genStr = makeBasicListPicker("descriptions","description")
			apis.writeToMod(apis.generatedFolder,"create_description")
			fs.writeFileSync(apis.generatedFolder+"/create_description.rs",genStr)

		}
	}
}