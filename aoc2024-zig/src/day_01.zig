const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}).init;
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const file = @embedFile("inputs/day_01.txt");

    var left: std.ArrayList(i32) = .empty;
    defer left.deinit(allocator);
    var right: std.ArrayList(i32) = .empty;
    defer right.deinit(allocator);

    var iter = std.mem.tokenizeAny(u8, file, " \n");
    var i: usize = 0;
    while (iter.next()) |s| {
        if (std.mem.eql(u8, s, ""))
            continue;

        const n = try std.fmt.parseInt(i32, s, 10);
        if (i % 2 == 0) {
            try left.append(allocator, n);
        } else {
            try right.append(allocator, n);
        }

        i += 1;
    }

    std.mem.sort(i32, left.items, {}, std.sort.asc(i32));
    std.mem.sort(i32, right.items, {}, std.sort.asc(i32));

    var stdout = std.fs.File.stdout().writer(&.{}).interface;
    try stdout.print("Part One: {}\n", .{partOne(left, right)});
    try stdout.print("Part Two: {}\n", .{partTwo(left, right)});
}

fn partOne(left: std.ArrayList(i32), right: std.ArrayList(i32)) u32 {
    var sum: u32 = 0;
    for (left.items, right.items) |l, r|
        sum += @abs(l - r);
    return sum;
}

fn partTwo(left: std.ArrayList(i32), right: std.ArrayList(i32)) u32 {
    var score: u32 = 0;
    for (left.items) |l|
        score += @as(u32, @intCast(l)) * @as(u32, @intCast(std.mem.count(i32, right.items, &[_]i32{l})));
    return score;
}
