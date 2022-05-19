//? Free to use ;)

const winjs = require('../src/index');// use 'winjsgui' instead

let window = new winjs.Window("Example", [800, 450]);

let content = window.setContent("Hello World");

window.launch(content);