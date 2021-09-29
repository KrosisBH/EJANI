To be filled...

EJANI is a tool I made to automatically set up my downloaded shows for language learning purposes. What it does is it removes all form of English from a .mkv file (audio dubs and subtitles) and automatically adds your subtitles (in your target language). 

Right now, the only language that will work is Japanese, once I get it up and running, I will try to make it language agnostic. 

To build, you need Rust. The build process is simple
cargo build --release

There are also two runtime dependencies:
ffmpeg, which comes with most Linux distributions. On Debian based systems, this is installed sudo apt install ffmpeg
ffsubsync, which you can get here https://github.com/smacke/ffsubsync

I (think) it should be OS agnostic, but I can only confirm that Linux systems work. 
