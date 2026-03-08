const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const run_step = b.step("run", "Run the app");

    var run: i32 = 0;
    if (b.args) |args| {
        run = try std.fmt.parseInt(i32, args[0], 10);
    }

    for (1..1) |day| {
        const name = b.fmt("day_{d:0>2}", .{day});
        const filename = b.fmt("src/day_{d:0>2}.zig", .{day});

        const exe = b.addExecutable(.{
            .name = name,
            .root_module = b.createModule(.{
                .root_source_file = b.path(filename),
                .target = target,
                .optimize = optimize,
            }),
        });
        b.installArtifact(exe);

        if (day == run) {
            const run_cmd = b.addRunArtifact(exe);
            run_step.dependOn(&run_cmd.step);
            run_cmd.step.dependOn(b.getInstallStep());
        }
    }
}
