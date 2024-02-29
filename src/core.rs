use crossbeam::channel::unbounded;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
use subprocess::{Popen, PopenConfig, PopenError, Redirection};

pub struct Job {
    pub num_workers: u8,
    pub filenames:   Vec<PathBuf>,
    pub format:      String,
    pub scale:       f32,
    pub compression: u8,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            filenames:   vec![],
            num_workers: 2,
            format:      "mp4".to_owned(),
            scale:       0.75,
            compression: 23,
        }
    }
}

impl Job {
    pub fn new(filenames: Vec<PathBuf>) -> Self {
        // todo: builder pattern or something to handle optional args
        Self {
            filenames: filenames,
            ..Default::default()
        }
    }

    pub fn run(&self) {
        let start = Instant::now();
        let multi = MultiProgress::new();
        let bar = multi
            .add(ProgressBar::new(self.filenames.len() as u64))
            .with_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] {spinner} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap()
                .tick_chars(SPINNER_CHARS)
                .progress_chars("##-"),
            );
        bar.enable_steady_tick(Duration::from_millis(100));
        multi.println("starting!").unwrap();

        // unbounded channel means its length is not limited.
        // We can append all the filenames all at once.
        let (tx, rx) = unbounded::<PathBuf>();

        let mut threads = vec![];
        for _ in 0..self.num_workers {
            // this is pretty ugly. There must be a better way.
            // Maybe you can assemble a queue of Task objects
            // which have all their dependent objects already
            // "baked in"
            let rx = rx.clone();
            let format = self.format.clone();
            let compression = self.compression.clone();
            let scale = self.scale.clone();
            let multi = multi.clone();
            let bar = bar.clone();
            threads.push(thread::spawn(move || {
                while let Ok(filename) = rx.try_recv() {
                    let spinner = multi.add(
                        ProgressBar::new_spinner()
                            .with_message(filename.to_str().unwrap().to_owned())
                            .with_style(
                                ProgressStyle::default_spinner()
                                    .tick_chars(SPINNER_CHARS),
                            ),
                    );
                    spinner.enable_steady_tick(Duration::from_millis(100));
                    let task_name = get_task_name(&filename.clone(), &format);
                    match compress(
                        &filename.clone(),
                        &format,
                        compression,
                        scale,
                    ) {
                        Ok(_) => {
                            bar.inc(1);
                            multi
                                .println(format!("✅ {task_name}"))
                                .unwrap();
                        }
                        Err(e) => panic!("Error: {e}"),
                    }
                }
            }))
        }

        for filename in self.filenames.iter() {
            tx.send(filename.clone())
                .expect("Error sending filename to queue!");
        }
        for thread in threads {
            let _ = thread.join();
        }
        bar.finish_with_message("done!");
        let duration = Instant::now() - start;
        println!("Finished in {} s", duration.as_millis() as f64 / 1000.0);
    }
}
fn compress(
    input: &PathBuf,
    format: &str,
    compression: u8,
    scale: f32,
) -> Result<(), PopenError> {
    let output = get_output_filename(input, format);
    let mut p = Popen::create(
        &[
            "ffmpeg",
            "-i",
            input.to_str().unwrap(),
            "-vcodec",
            "libx264", // this is about 10x faster than libx265...
            "-preset",
            "ultrafast",
            "-crf",
            format!("{}", compression).as_str(),
            // This scale filter essentially divides the video dimensions by 2.
            // This could be achieved by a simple `scale=ceil(iw/2):ceil(ih/2)`
            // filter. However, the codec libx264 requires the final dimensions
            // to be divisible by 2 also. So we divide by 4, truncate the
            // result to an int, and multiply by 2; thus dividing
            // by 2 _and_ making sure the result is divisible by 2.
            "-vf",
            format!(" scale=trunc(iw*{}/2)*2:trunc(ih*{}/2)*2 ", scale, scale)
                .as_str(),
            &output,
            "-y",
        ],
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    if let Ok(status) = p.wait() {
        let (out, err) = p.communicate(None)?;
        if !status.success() {
            if let Some(out) = out {
                println!("out={out}");
            }
            if let Some(err) = err {
                panic!("{}", ERR_STR.replace("{ERR}", &err.to_string()));
            }
        }
    }
    Ok(())
}

fn get_output_filename(
    filename: &PathBuf,
    format: &str,
) -> String {
    let stem = filename
        .parent()
        .unwrap()
        .join(filename.file_stem().unwrap());
    format!("{}_compressed.{}", stem.to_str().unwrap(), format)
}

fn get_task_name(
    filename: &PathBuf,
    format: &str,
) -> String {
    format!(
        "{} -> {}",
        filename.to_str().unwrap(),
        get_output_filename(filename, format),
    )
}

const ERR_STR: &str = "
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
Something went wrong with ffmpeg! Here's the error:
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

{ERR}
";
const SPINNER_CHARS: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";
