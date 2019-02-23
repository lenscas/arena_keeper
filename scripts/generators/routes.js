module.exports = (apis) => {
	const routes = apis.getRoutes()
	const fs = require("fs");
	const text = `
#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub enum Windows{
	${routes.join(",\n\t")}
}
`
	apis.writeToMod(apis.generatedFolder, "routes")
	const toStore = apis.generatedFolder + "/routes.rs"
	fs.writeFileSync(toStore,text)
	const file = apis.loadTemplate("active_windows")
	fs.writeFileSync(
		apis.generatedFolder+"/active_windows.rs",
		file.replace(
			/{{LOAD_CRATES}}/g,
			routes.map(
				v=> "use crate::pages::"+v.toLowerCase()+"_window::"+v.toLowerCase()+"::"+v+";\n"
			).join("")
		).replace(/{{RENDER_WINDOWS}}/g,
			routes.map(
				v=>"Windows::"+v+"=> html!{<"+v+":/>}"
			).join(",")
		)
	)
	apis.writeToMod(apis.generatedFolder,"active_windows")
}