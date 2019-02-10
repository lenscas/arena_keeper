const fs = require('fs');
const windowName = process.argv[2]

const template = fs.readFileSync(__dirname+"/templates/window.rs","utf-8")

const firstAsUpper = windowName.charAt(0).toUpperCase() + windowName.substr(1);
const filledIn = template.replace(/{{WINDOW_NAME_CAPS}}/g,firstAsUpper).replace(/{{WINDOW_NAME}}/g, windowName)

const asArr = __dirname.split("/")
asArr.pop()
asArr.push("src")
asArr.push("pages")
fs.mkdirSync(asArr.join("/")+ "/"+windowName)
fs.appendFileSync(asArr.join("/")+"/mod.rs","\npub mod "+windowName+";\n");
asArr.push(windowName)
fs.writeFileSync(asArr.join("/")+"/mod.rs", "pub mod "+windowName+";\n")
fs.writeFileSync(asArr.join("/")+"/"+windowName+".rs",filledIn)

const routes = JSON.parse(fs.readFileSync(__dirname+"/routes.json"))
routes.push(firstAsUpper)
fs.writeFileSync(__dirname+"/routes.json",JSON.stringify(routes))
console.log(filledIn)