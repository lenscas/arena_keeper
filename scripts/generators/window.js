module.exports = (apis) => {
	const fs = require("fs")
	const windowName = process.argv[3]
	const folderName = windowName + "_window"
	const template = apis.loadTemplate("window")
	const firstAsUpper = apis.firstToUpper(windowName)
	const filledIn = template.replace(/{{WINDOW_NAME_CAPS}}/g,firstAsUpper).replace(/{{WINDOW_NAME}}/g, windowName)
	const windowFolder = apis.appendFile("..","src","pages",folderName)
	console.log("Make folder")
	fs.mkdirSync(windowFolder)
	apis.writeToMod(windowFolder+"/..",folderName)
	apis.writeToMod(windowFolder, windowName)
	fs.writeFileSync(windowFolder+"/"+windowName+".rs",filledIn)
	apis.addToRoutes(firstAsUpper)
	console.log("Generate routes")
	apis.runGenerator("routes")
}
