# COmpress VIDeo
A tool for compressing videos. Thin wrapper around ffmpeg. Written in Rust :crab:

![example](/docs/covid-example.gif)

## Installation

### MacOS
You will need to install ffmpeg: 
```shell 
brew install ffmpeg 
```

then run: 
```shell
# download the binary and rename it `covid`
curl -L -o covid https://github.com/binnev/covid/releases/download/v0.1.0/covid_macos
chmod +x covid
mkdir -p ~/.local/bin # a place for user binaries that doesn't require sudo
mv covid ~/.local/bin # move the binary there
```
You'll need to make it so your shell can see the binary from anywhere:
```shell
echo export PATH=\"$HOME'/.local/bin:$PATH'\" >> ~/.zshrc  # or whatever your shell config file is
```

Type `exit` in all your open terminals, and open a new terminal. You should be able to run `covid` anywhere.

## Usage 
```shell
covid --help

Usage: covid [OPTIONS] <filenames>...

Arguments:
  <filenames>...  One or more filenames e.g. `foo.mov bar.mov` your shell may also support wildcards e.g. `*.mov`

Options:
  -f, --format <VALUE>       Format to convert to e.g. `mp4`
  -s, --scale <VALUE>        Scale factor used to resize the video e.g. `0.75`
  -c, --compression <VALUE>  Compression factor used by the libx264 codec. Integer between 0-51
  -n, --num-workers <VALUE>  Number of workers to use (default=1)
  -h, --help                 Print help
  -V, --version              Print version
  ```
