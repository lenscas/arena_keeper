let types = {
	"elf" : [
		"Astra",
	],
	human : [
		"Random name",
	],
	"centaur" : [
		"Ferold Firemane"
	],
	"chaos elemental" : [
		"Virvir Dreameater"
	],
	"giant" :[
		"Badaghol"
	],
	"orc" : [
		"Vuul n'gh Vuul"
	],
	"lich" : [
		"Zamajin the Unburied"
	],
	"dward" : [
		'Lorron "the Avalanche" Mirreksshield'
	]
}
function geneateName(type){
	console.log(type)
	let possibles = types[type.toLowerCase()]
	if(!possibles){
		possibles = [
			"Tyler",
			"Lisy",
			"Lumaceon",
		]
	}
	return possibles[Math.floor(Math.random() * possibles.length)];
}