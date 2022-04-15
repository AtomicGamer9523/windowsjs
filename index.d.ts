/**
 * @author AtomicGamer9523 
 * @license MIT
*/

type f64 = number

export interface window {
    title: string,
    window_size: [len: f64, width:f64]
}

interface HelloState {
    name: string,
}

/**
 * @class
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
     * @method launch
     * @returns {void}
    */
    public launch ( ) : void ;
}