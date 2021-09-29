# EJANI

EJANI is a tool I made to automatically set up my downloaded shows for language learning purposes. What it does it automatically syncs and adds subtitles to your series of videos. 


Right now, the only language that will work is Japanese, once I get it up and running, I will try to make it language agnostic. 

## Building
To build, you need Rust. The build process is simple.

`git clone https://github.com/KrosisBH/EJANI`  
`cd EJANI`  
`cargo build --release`  

The binary is in /target/release/

There are also two runtime dependencies:
ffmpeg, which comes with most Linux distributions. On Debian based systems, this is installed `sudo apt install ffmpeg`  

ffsubsync, which you can get here https://github.com/smacke/ffsubsync

## How to use

`ejani -ia <path_to_show_folder> -is <path_to_subtitles> -o <path_to_output>`

This is only test to work on Linux. I tried to use cross-platform operations, but it remains untested on MacOS, Windows, and BSD systems.

I personally use this with Anki + mpv + https://github.com/Ajatt-Tools/mpvacious, which makes making flashcards off what I watch very trivial

## License
EJANI is released under the BSD-3-Clause license, please see the license file for details.

## Closing
This is my first publically released piece of software! I'm excited to release this and hope it aids you. I am planning more features, such as more file formats and eventually supporting languages besides Japanese. 

Please let me know if there's anything you'd like to see from this!
