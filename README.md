video4discord
--------
A tool for encoding videos to be < 8 MiB
```
USAGE:
    video4discord [OPTIONS] -i <INPUT_FILE>

OPTIONS:
    -a, --audio-bitrate <AUDIO_BITRATE>        audio bitrate in Kbps [default: 32]
    -d, --div <DIV>                            factor to divide X/Y resolution by [default: 2]
    -h, --help                                 Print help information
    -i <INPUT_FILE>                            
    -m, --muxing-overhead <MUXING_OVERHEAD>    muxing overhead in percent [default: 5]
    -o <OUTPUT_FILE>                           [default: out.mp4]
    -t, --target-filesize <TARGET_FILESIZE>    target filesize in MiB [default: 8]
    -V, --version                              Print version information

```

```
$ video4discord -i input.mp4 -d 4 -o output.mp4
aiming for filesize < 8MiB
scaling video down to 1/4 x/y resolution
new audio bitrate: 32k
new video bitrate: 738k
running ffmpeg pass 1
running ffmpeg pass 2


$ du -h input.mp4
83M	input.mp4

$ du -h output.mp4
7.6M	output.mp4
```
