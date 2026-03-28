//// [tests/cases/compiler/extendedUnicodePlaneIdentifiersJSDoc.ts] ////

//// [file.js]
/**
 * Adds
 * @param {number} 𝑚
 * @param {number} 𝑀
 */
function foo(𝑚, 𝑀) {
    console.log(𝑀 + 𝑚);
}

//// [file.js]
"use strict";
/**
 * Adds
 * @param {number} 𝑚
 * @param {number} 𝑀
 */
function foo(𝑚, 𝑀) {
    console.log(𝑀 + 𝑚);
}
