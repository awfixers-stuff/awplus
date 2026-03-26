pub const JestPrettyFormat = struct {
    pub const FormatOptions = struct {
        enable_colors: bool = false,
        add_newline: bool = false,
        flush: bool = false,
        quote_strings: bool = false,
    };

    pub fn format(
        _: enum { Debug },
        globalObject: *jsc.JSGlobalObject,
        values: [*]const jsc.JSValue,
        len: usize,
        out: anytype,
        _: FormatOptions,
    ) !void {
        _ = globalObject;
        _ = values;
        _ = len;
        try out.writeAll("<unknown>");
        try out.flush();
    }
};

const jsc = @import("bun").jsc;
