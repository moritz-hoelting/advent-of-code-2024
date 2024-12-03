const std = @import("std");
const indexOf = std.mem.indexOf;

const problemInput = @embedFile("./input.txt");

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    const part1Result = try part1(problemInput);
    try stdout.print("Part 1: {}\n", .{part1Result});
    const part2Result = try part2(problemInput);
    try stdout.print("Part 2: {}\n", .{part2Result});

    try bw.flush();
}

fn toInt(slice: []const u8) !u32 {
    return std.fmt.parseInt(u32, slice, 10);
}

fn part1(input: []const u8) !u32 {
    var start: usize = 0;
    var sum: u32 = 0;
    while (indexOf(u8, input[start..], "mul(")) |found| {
        start += found;

        var i: usize = 1;
        inner: while (i < 4) {
            if (input[start + 3 + i] >= '0' and input[start + 3 + i] <= '9') {
                i += 1;
            } else {
                break :inner;
            }
        }

        const a = if (i > 1) try toInt(input[start + 4 .. start + 3 + i]) else @as(u32, 0);

        if (input[start + 3 + i] != ',') {
            start += 5 + i;
            continue;
        }

        var j: usize = 1;
        inner: while (j < 4) {
            if (input[start + 4 + i + j] >= '0' and input[start + 4 + i + j] <= '9') {
                j += 1;
            } else {
                break :inner;
            }
        }

        const b = if (i > 1) try toInt(input[start + 4 + i .. start + 4 + i + j]) else @as(u32, 0);

        start += 5 + i + j;

        if (input[start - 1] == ')') {
            sum += a * b;
        }
    }

    return sum;
}

fn part2(input: []const u8) !u32 {
    var start: usize = 0;
    var sum: u32 = 0;
    var enabled = true;
    while (indexOf(u8, input[start..], "mul(")) |foundA| {
        var found = foundA;
        if (indexOf(u8, input[start..], "do")) |foundB| {
            found = @min(foundA, foundB);
        }

        start += found;

        if (input[start] == 'd') {
            if (std.mem.eql(u8, input[start .. start + 4], "do()")) {
                enabled = true;
                start += 4;
            } else if (std.mem.eql(u8, input[start .. start + 7], "don't()")) {
                enabled = false;
                start += 7;
            } else {
                start += 2;
            }
        } else {
            var i: usize = 1;
            inner: while (i < 4) {
                if (input[start + 3 + i] >= '0' and input[start + 3 + i] <= '9') {
                    i += 1;
                } else {
                    break :inner;
                }
            }
            const a = if (i > 1) try toInt(input[start + 4 .. start + 3 + i]) else @as(u32, 0);

            if (input[start + 3 + i] != ',') {
                start += 5 + i;
                continue;
            }

            var j: usize = 1;
            inner: while (j < 4) {
                if (input[start + 4 + i + j] >= '0' and input[start + 4 + i + j] <= '9') {
                    j += 1;
                } else {
                    break :inner;
                }
            }

            const b = if (i > 1) try toInt(input[start + 4 + i .. start + 4 + i + j]) else @as(u32, 0);

            start += 5 + i + j;

            if (enabled and input[start - 1] == ')') {
                sum += a * b;
            }
        }
    }

    return sum;
}

const sample1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const sample2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

test "part 1" {
    const result = try part1(sample1);
    try std.testing.expectEqual(@as(u32, 161), result);
}

test "part 2" {
    const result = try part2(sample2);
    try std.testing.expectEqual(@as(u32, 48), result);
}
