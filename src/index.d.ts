/**
 * @author AtomicGamer9523 
 * @license MIT
*/
module.exports = require('../dist');
exports = require('../dist');


/**
 * @interface window
 * @public 
 * @exports window
 * @property {string} title
 * @property {window["window_size"]} window_size
*/
export interface window {
    /**
     * @public
     * @type {string}
    */
    title: string,
    /**
     * @public
     * @type {window["window_size"]}
    */
    window_size: [len: number, width:number]
}

/**
 * @class Window
 * @public
 * @exports Window
 * @implements {window}
*/
export class Window implements window {

    /**
     * @private
     * @type {window["title"]}
    */
    title : window["title"] ;
    

    /**
     * @private
     * @type {window["window_size"]}
    */
    window_size: window["window_size"] ;

    /**
     * @public
     * @constructor
     * @param {window["title"]} title
     * @param {window["window_size"]} size
    */
    constructor ( title : window["title"], size ? : window["window_size"] ) ;

    /**
     * @public
     * @method setContent
     * @param
     * @returns {Widget<MainState>}
    */
    public setContent( text : string ) : Widget < MainState > ;
    

    /**
     * @public
     * @method launch
     * @returns {void}
    */
    public launch ( content: Widget<MainState> ) : void ;
}









class Widget<State> {
    private st: State;
}

class MainState {
    fdata: string
}