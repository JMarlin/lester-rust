var ffi = require('ffi');

var lib = ffi.Library('target/release/liblester', {
		'process': ['void', []]
	});
	
lib.process();

console.log("done!");
