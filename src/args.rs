use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    let matches = Command::new("pngit")
        .propagate_version(true)
        .version("0.1.0")
        .about("Hide a message inside an png file - pngit!")
        .subcommand(
            Command::new("encode")
                .about("encode a file based on the specified chunk type")
                .arg(Arg::new("filepath").help("the filepath of png to encode").required(true))
                .arg(Arg::new("chunk type").required(true))
                .arg(Arg::new("message").required(true))
                .arg(Arg::new("output path").required(false)),
        )
        .subcommand(
            Command::new("decode")
                .about("decode the png file")
                .arg(Arg::new("filepath"))
                .arg(Arg::new("chunk type")),
        )
        .subcommand(
            Command::new("remove")
                .about("remove a specific chunk type from a png file")
                .arg(Arg::new("filepath"))
                .arg(Arg::new("chunk type")),
        )
        .subcommand(
            Command::new("print")
                .about("print the bytes")
                .arg(Arg::new("filepath")),
        )
        .get_matches();
    matches
}
