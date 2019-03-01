
$("html").on("mousedown","#active",function(e) {
	dragMouseDown(e)
})
$("html").on("mousedown","#window-header",function(e) {
	$(".window").removeAttr('id')
	$(e.target).closest(".window").attr("id","active")
	dragMouseDown(e)
})
$("html").on("click",".window",function(e) {
	$(".window").removeAttr('id')
	$(e.target).closest(".window").attr("id","active")
})
function dragMouseDown(e) {
	e = e || window.event;
	e.preventDefault();
	// get the mouse cursor position at startup:
	let pos1 = 0;
	let pos2 = 0;
	let pos3 = e.clientX;
	let pos4 = e.clientY;
	const elmnt = document.getElementById("active");
	const closeDragElement = function () {
		// stop moving when mouse button is released:
		document.onmouseup = null;
		document.onmousemove = null;
	}
	const  elementDrag = function(e) {
		e = e || window.event;
		e.preventDefault();
		// calculate the new cursor position:
		pos1 = pos3 - e.clientX;
		pos2 = pos4 - e.clientY;
		pos3 = e.clientX;
		pos4 = e.clientY;
		// set the element's new position:
		elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
		elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
	}
	document.onmouseup = closeDragElement;
	// call a function whenever the cursor moves:
	document.onmousemove = elementDrag;
}
