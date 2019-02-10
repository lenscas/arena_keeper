const fs = require("fs");
const content = JSON.parse(fs.readFileSync(__dirname+"/routes.json"));
const asArr = __dirname.split("/")
asArr.pop()
asArr.push("src")
asArr.push("generated")
asArr.push("routes.rs")
const toStore = asArr.join("/")
const text = `
#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub enum Windows{
	${content.join(",\n\t")}
}
`
asArr.pop()
asArr.push("mod.rs")
const modFile = asArr.join("/")
if (fs.existsSync(modFile)) {
	if(fs.readFileSync(modFile,"utf-8").indexOf("pub mod routes;") === -1 ){
		fs.appendFileSync(modFile,"\npub mod routes;\n")
	}

} else {
	fs.writeFileSync(modFile,"pub mod routes;\n")
}
fs.writeFileSync(toStore,text)

const file = fs.readFileSync(__dirname+"/templates/active_windows.rs","utf-8")
asArr.pop()
asArr.push("active_windows.rs")
fs.writeFileSync(
	asArr.join("/"),
	file.replace(
		/{{LOAD_CRATES}}/g,
		content.map(
			v=> "use crate::pages::"+v.toLowerCase()+"::"+v.toLowerCase()+"::"+v+";\n"
		).join("")
	).replace(/{{RENDER_WINDOWS}}/g,
		content.map(
			v=>"Windows::"+v+"=> html!{<"+v+":/>}"
		).join(",")
	)
)
if(fs.readFileSync(modFile,"utf-8").indexOf("pub mod active_windows;") === -1 ){
	fs.appendFileSync(modFile,"\npub mod active_windows;\n")
}

console.log(toStore)
console.log(content)