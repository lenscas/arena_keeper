function getFromList(listName,typeNum){
	const list = lists[listName][typeNum];
	return list[getMax(list.length)]
}

function randomIntFromInterval(min,max) // min and max included
{
    return Math.floor(Math.random()*(max-min+1)+min);
}

function getMax(max) {
	return randomIntFromInterval(0,max - 1) // Math.floor(Math.random() * max)
}
