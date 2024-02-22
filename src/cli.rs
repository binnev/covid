use std::path::PathBuf;

use clap::arg;
use clap::command;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::Command;

pub fn cli() -> Command {
    // https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html
    command!()
        .arg(
            Arg::new("filenames")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .help("One or more filenames e.g. `foo.mov bar.mov` your shell may also support wildcards e.g. `*.mov`"),
        )
        .arg(
            arg!(-f --format <VALUE> "Format to convert to e.g. `mp4`")
                .required(false)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(-s --scale <VALUE> "Scale factor used to resize the video e.g. `0.75`")
                .value_parser(value_parser!(f32))
                .required(false)
        )
        .arg(
            arg!(-c --compression <VALUE> "Compression factor used by the libx264 codec. Integer between 0-51")
                .value_parser(value_parser!(u8).range(0..=51))
                .required(false)
        )
        .arg(
            arg!(-n --"num-workers" <VALUE> "Number of workers to use (default=1)")
                .required(false)
                .value_parser(value_parser!(u8)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        let cmd = cli();
        let matches = cmd.get_matches_from(vec![
            "covid", "*.mov", "foo.avi", "--format", "mp4", "-n", "4", "-s",
            "0.69", "-c", "23",
        ]);

        let filenames: Vec<String> = matches
            .get_many::<PathBuf>("filenames")
            .expect("We know we passed some paths above")
            .into_iter()
            .map(|path| {
                path.clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
            .collect();
        let expected: Vec<String> = vec!["*.mov", "foo.avi"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        assert_eq!(filenames, expected);

        let format = matches
            .get_one::<String>("format")
            .unwrap();
        assert_eq!(format, "mp4");

        let n: u8 = *matches
            .get_one::<u8>("num-workers")
            .unwrap();
        assert_eq!(n, 4);

        let scale: f32 = *matches.get_one::<f32>("scale").unwrap();
        assert_eq!(scale, 0.69);

        let compression: u8 = *matches
            .get_one::<u8>("compression")
            .unwrap();
        assert_eq!(compression, 23);
    }
}
