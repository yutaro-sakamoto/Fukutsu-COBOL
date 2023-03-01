const wasm = require("./pkg");

let core = wasm.CobolCore.new_by_string("helloworld");
let hello_field = core.register_field(
    0,
    5,
    wasm.CobolFieldType.Alphanumeric,
    0,
    0,
    wasm.FLAG_NONE,
    "",
);
let world_field = core.register_field(
    5,
    5,
    wasm.CobolFieldType.Alphanumeric,
    0,
    0,
    wasm.FLAG_NONE,
    "",
);

console.log("before move====");
console.log("hello: " + core.field_as_string(hello_field));
console.log("world: " + core.field_as_string(world_field));

core.move_field(hello_field, world_field);

console.log("after move====");
console.log("hello: " + core.field_as_string(hello_field));
console.log("world: " + core.field_as_string(world_field));
