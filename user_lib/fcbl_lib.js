
const wasm = require("./fcbl-nodejs");

module.exports.TYPE_ALPHANUMERIC = wasm.TYPE_ALPHANUMERIC;
module.exports.FLAG_NONE = wasm.FLAG_NONE;

module.exports.get_fcbl_core = function (size) {
    return wasm.CobolCore.new(size);
}

module.exports.register_field = function (core, start_index, len, typ, digits, scale, flags, pic) {
    return core.register_field(start_index, len, typ, digits, scale, flags, pic);
}

module.exports.set_string = function (core, field, value) {
    return core.set_string(field, value);
}

module.exports.move_field = function (core, src, dst) {
    return core.move_field(src, dst);
}

module.exports.field_as_string = function (core, field) {
    return core.field_as_string(field);
}

module.exports.display = function (field) {
    console.log(field);
}