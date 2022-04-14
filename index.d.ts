/**
 * @author AtomicGamer9523 
 * @license MIT
*/



/**
 * @class
 * @public
 * @exports MyClass
 * @param {number} val
*/
export class MyClass {

    /**
     * @private
     * @type {number}
    */
    private val : number ;

    /**
     * @public
     * @constructor
     * @param {number} val 
    */
    constructor ( val : number ) ;

    /**
     * @public
     * @method pl1
     * @returns {void}
    */
    public pl1 ( ) : void ;

    /**
     * @public
     * @method v
     * @returns {number}
    */
    public get v() : number ;

    /**
     * @public
     * @method setVal
     * @param {number} newval
     * @returns {void}
    */
    public setVal( newval : number ) : void ;
}