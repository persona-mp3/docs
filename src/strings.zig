const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const username = "persona-mp3 zigging";
    var player_name: []const u8 = undefined;
    player_name = "persona-mp3 zigging";

    // this compares the types instead of the values
    // infact this is a comp-time error
    //
    //  my-zig/book  | zig run src/strings.zig
    // src/strings.zig:9:18: error: operator == not allowed for type '[
    // ]const u8'
    //     if (username == player_name) {
    //         ~~~~~~~~~^~~~~~~~~~~~~~
    // referenced by:
    //     callMain [inlined]: /opt/homebrew/Cellar/zig/0.15.2/lib/zig/
    const is_eql = std.mem.eql(u8, username, player_name);
    if (is_eql) {
        print("username and player name are equal\n", .{});
    } else {
        print("username and {s} and {s} are not equal\n", .{ username, player_name });
    }
}
