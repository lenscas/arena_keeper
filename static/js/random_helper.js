function getFromList(listName,typeNum){
	const list = lists[listName][typeNum];
	return list[getMax(list.length)]
}
function getMax(max) {
	return Math.floor(Math.random() * max)
}
