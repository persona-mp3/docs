const std = @import("std");
const imp = @import("root.zig");
const print = std.debug.print;

pub fn main() !void {
    const username = "Pickle Rick";
    print("{s} in the building!\n", .{username});

    // so you can actually import funcs from the
    // root.zig file. docs said it's used when using
    // libraries?
    try imp.bufferedPrint();

    // All objects declared with 'var' must be mutated
    // and have an explicit type
    var age: u32 = 78;
    print("this shouldn't compile? {d}\n", .{age});
    age = 20;

    // but this compiles?
    // I mean the compiler can infer this type at comp_time i guess?
    var player_tag = "mighty_pickle_rick";
    player_tag = undefined;

    // Got this error
    //
    // src/main.zig:27:20: error: expected type '*const [14:0]u8', found
    // '*const [23:0]u8'
    //     network_addr = "understanding_zig_part2";
    //                    ^~~~~~~~~~~~~~~~~~~~~~~~~
    // src/main.zig:27:20: note: pointer type child '[23:0]u8' cannot c
    // ast into pointer type child '[14:0]u8'
    // src/main.zig:27:20: note: array of length 23 cannot cast into an
    //  array of length 14
    // var network_addr = "localhost:3000";
    // print("starting server on {s}\n", .{network_addr});
    // network_addr = "understanding_zig_part2";
    // So strings are simply array bytes like in Golang?
    // And by default, these are very tricky to be mutable?

    print("\n\n === strings === \n\n", .{});
    var source_material: []const u8 = undefined;
    source_material = "say my name!";
    print("  WALTER: {s}\n", .{source_material});
    source_material = "HEISEINGBERG";
    print("  RANDOM: {s}\n", .{source_material});
    source_material = "he can't keep getting away with this";
    print("  JESSE-PINKMAN: {s}\n", .{source_material});
    // You can do this, but before and after must be the same length
    var zing = "zing_zog";
    zing = "zoo_ginz";
    print("foo {s}\n ", .{zing});

    print("\n\n === ARRAYS === \n\n", .{});
    // ARRAYS
    const trial = [_]u8{ '9', '0', 8 }; // this is interesting
    const slices = trial[0..3];
    print("slices? {any}\n", .{slices});
    print("slices_length: {d}\n", .{slices.len});

    print("to concat arrays -> a++b -> {any}\n", .{slices ++ trial});
}
