
import * as wasm from "fukutsu-cobol";
let core = wasm.CobolCore.new_by_string("hello_world");
let field_ab = core.register_field (0, 5, wasm.CobolFieldType.Alphanumeric, 0, 0, wasm.FLAG_NONE, "");
let field_cd = core.register_field (5, 5, wasm.CobolFieldType.Alphanumeric, 0, 0, wasm.FLAG_NONE, "");
core.move_field (field_ab, field_cd);
console.log (core.field_as_string (field_cd));
console.log (core.field_as_string (field_ab));