const fs = require('fs');

const generatorName = process.argv[2]
const currentFolder = __dirname.split("/")

const appendFile = (...args) => currentFolder.map(v=>v).join("/")+"/"+args.join("/")

const routeFilePath = appendFile("routes.json")
const generatedJSPath = appendFile("..","static","js","generated")

const templates = appendFile("templates")
const generatedFolder = appendFile("..","src","generated")
const loadTemplate = temp => fs.readFileSync(templates+"/" +temp+".rs","utf-8")
const firstToUpper = (str) => str.charAt(0).toUpperCase() + str.substr(1);
const getRoutes = ()=> {
	return JSON.parse(fs.readFileSync(routeFilePath,"utf-8"))

}
const addToRoutes = (route) =>{
	const routes = getRoutes()
	const routeFilePath = appendFile("routes.json")
	routes.push(route)
	fs.writeFileSync(routeFilePath,JSON.stringify(routes))
}
const writeToMod = (location,toAdd) => {
	const modFile = location + "/mod.rs"
	const fullToAdd = "pub mod " + toAdd+";\n"
	if (fs.existsSync(modFile)) {
		if(fs.readFileSync(modFile,"utf-8").indexOf(fullToAdd) === -1 ){
			fs.appendFileSync(modFile,fullToAdd)
		}

	} else {
		fs.writeFileSync(modFile,fullToAdd)
	}
}
const runGenerator =(genName) => {
	const generator = require("./generators/"+genName)
	generator({
		templates,
		generatedFolder,
		generatedJSPath,
		appendFile,
		loadTemplate,
		firstToUpper,
		writeToMod,
		getRoutes,
		addToRoutes,
		runGenerator
	})
}
runGenerator(generatorName)