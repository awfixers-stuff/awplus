//// [tests/cases/compiler/unicodeEscapesInNames02.ts] ////

//// [extendedEscapesForAstralsInVarsAndClasses.ts]
// U+102A7 CARIAN LETTER A2
declare var 𐊧: string;
declare var \u{102A7}: string;

if (Math.random()) {
    𐊧 = "hello";
}
else {
    \u{102A7} = "hallo";
}

class Foo {
    \u{102A7}: string;
    constructor() {
        this.\u{102A7} = " world";
    }
    methodA() {
        return this.𐊧;
    }
}

export var _𐊧 = new Foo().\u{102A7} + new Foo().methodA();

_\u{102A7} += "!";

//// [astralAsSurrogatePair.ts]
import { _𐊧 as \uD800\uDEA7 } from "./extendedEscapesForAstralsInVarsAndClasses.js";


//// [extendedEscapesForAstralsInVarsAndClasses.js]
if (Math.random()) {
    𐊧 = "hello";
}
else {
    \u{102A7} = "hallo";
}
class Foo {
    constructor() {
        this.\u{102A7} = " world";
    }
    methodA() {
        return this.𐊧;
    }
}
export var _𐊧 = new Foo().\u{102A7} + new Foo().methodA();
_\u{102A7} += "!";
//# sourceMappingURL=extendedEscapesForAstralsInVarsAndClasses.js.map
//// [astralAsSurrogatePair.js]
export {};
//# sourceMappingURL=astralAsSurrogatePair.js.map