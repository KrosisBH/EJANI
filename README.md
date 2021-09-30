# EJANI

EJANI is a tool I made to automatically set up my downloaded shows for language learning purposes. What it does it automatically syncs and adds subtitles to your series of videos. 


Right now, the only language that will work is Japanese, once I get it up and running, I will try to make it language agnostic. 

## Building

### Unix-Likes (Linux, MacOS, BSDs)
To build, you need Rust. The build process is simple.

`git clone https://github.com/KrosisBH/EJANI`  
`cd EJANI`  
`cargo build --release`  

The binary is in /target/release/

There are also two runtime dependencies:
ffmpeg, which comes with most Linux distributions. On Debian based systems, this is installed `sudo apt install ffmpeg`  

ffsubsync, which you can get here https://github.com/smacke/ffsubsync, which uses Python.

### Windows

Note: This program must be run from the command shell. I recommend the Windows Terminal you can get off the Microsoft store. 

The build process is the same. The binary is provided however. The runtime dependencies are the same, but you need to have them in your path. 

ffmpeg can be installed and added to your path easily with scoop.  
`scoop install ffmpeg`  

If not, you will have to download binaries from http://ffmpeg.org/ and manually add the bin folder to your path.  

For ffsubsync, the install process is the same. This requires Python, which isn't default on Windows, you can download it from the Microsoft Store. This will assume you have done that. For this to work properly, you need to add your Python scripts to the path. For me, this was at  
`C:\Users\<user>\AppData\Local\Packages\PythonSoftwareFoundation.Python.3.7_qbz5n2kfra8p0\LocalCache\local-packages\Python37\Scripts`.  

You'll know this is the right place if there is a 'ffs' execuable in there.  

After this, be sure that the shell you are working in has been restarted so that the paths load correctly. Try running the ffmpeg and ffs commands to make sure everything is good. If so, then you are ready.

## How to use

`ejani -ia <path_to_show_folder> -is <path_to_subtitles> -o <path_to_output>`

This is only test to work on Linux. I tried to use cross-platform operations, but it remains untested on MacOS, Windows, and BSD systems.

I personally use this with Anki + mpv + https://github.com/Ajatt-Tools/mpvacious, which makes making flashcards off what I watch very trivial

## License
EJANI is released under the BSD-3-Clause license, please see the license file for details.

## Closing
This is my first publically released piece of software! I'm excited to release this and hope it aids you. I am planning more features, such as more file formats and eventually supporting languages besides Japanese. 

Please let me know if there's anything you'd like to see from this!
