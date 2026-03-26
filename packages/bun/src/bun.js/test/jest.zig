pub const Jest = struct {
    pub const bun_test = struct {
        pub const FakeTimers = @import("./FakeTimers.zig").FakeTimers;
    };
};
