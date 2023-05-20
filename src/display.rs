use std::io::Write;

// sets terminal color by printing ANSI
// TODO maybe set an enum for this
pub fn set_color<T: Into<String>>(color: T) {
    let ansi = match color.into().as_str() {
        "black"     => "\x1b[30m",
        "red"       => "\x1b[31m",
        "green"     => "\x1b[32m",
        "yellow"    => "\x1b[33m",
        "blue"      => "\x1b[34m",
        "magenta"   => "\x1b[35m",
        "cyan"      => "\x1b[36m",
        "white"     => "\x1b[37m",
        "none"      => "\x1b[0m",
        _           => "\x1b[0m",
    };

    print!("{}", ansi);
}

pub fn write_continuous(chunk: &str) {
    print!("{}", chunk);

    // flush stdout to avoid newlines
    std::io::stdout()
        .flush()
        .unwrap();
}

pub fn write_error(err_str: &str) {
    set_color("red");
    eprintln!("The server responded with an error:\n");

    set_color("none");
    eprintln!("{}", err_str);
}
