mod cli;
mod core;
use core::Job;
use std::{path::PathBuf, process::exit};

fn main() {
    let cmd = cli::cli();
    let matches = cmd.get_matches();

    let filenames: Vec<PathBuf> = matches
        .get_many::<PathBuf>("filenames")
        .expect("Couldn't unpack filenames!")
        .into_iter()
        .map(|path| {
            if !path.exists() {
                println!("File does not exist: {:?}", path);
                exit(420);
            }
            path.clone()
        })
        .collect();

    let mut job = Job::new(filenames);

    if let Some(format) = matches.get_one::<String>("format") {
        job.format = format.clone();
    }
    if let Some(n) = matches.get_one::<u8>("num-workers") {
        job.num_workers = *n;
    }
    if let Some(scale) = matches.get_one::<f32>("scale") {
        job.scale = *scale;
    }
    if let Some(compression) = matches.get_one::<u8>("compression") {
        job.compression = *compression;
    }

    job.run()
}
