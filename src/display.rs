use std::io::Write;

// TODO use a better method
pub fn colorize(color: &str, text: &str) -> String {
    match color {
        "black"     => "\x1b[30m".to_owned() + text,
        "red"       => "\x1b[31m".to_owned() + text,
        "green"     => "\x1b[32m".to_owned() + text,
        "yellow"    => "\x1b[33m".to_owned() + text,
        "blue"      => "\x1b[34m".to_owned() + text,
        "magenta"   => "\x1b[35m".to_owned() + text,
        "cyan"      => "\x1b[36m".to_owned() + text,
        "white"     => "\x1b[37m".to_owned() + text,
        "none"      => "\x1b[0m".to_owned()  + text,
        _           => "\x1b[0m".to_owned()  + text,
    }
}

pub fn write_continuous(color: &str, chunk: &str) {

    print!("{}", colorize(&color, &chunk));

    // flush stdout to avoid newlines
    std::io::stdout()
        .flush()
        .unwrap();
}

pub fn write_error(err_str: &str) {
    println!(
        "{}\n",
        colorize(
            &"red",
            "The server responded with an error:"
        )
    );
    println!("{}", colorize(&"none", err_str));
}
