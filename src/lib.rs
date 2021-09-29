use std::{fs, io};
use std::process::Command;
use std::env;
pub struct EjaniInstance {
    
    //required fields
    path_to_anime : String,
    path_to_subs: String,
    output_path: String

}

impl EjaniInstance {

    pub fn new( path_to_anime: String,  path_to_subs: String, output_path: String) -> EjaniInstance {
        EjaniInstance{
            path_to_anime,
            path_to_subs,
            output_path
        }
    }

    pub fn run(self) -> io::Result<()>{

        let anime_paths = pull_and_alphabetize(self.path_to_anime, ".mkv")?;
        let mut sub_paths = pull_and_alphabetize(self.path_to_subs, ".srt")?;

        sub_paths.sort_by_key(|dir| dir.path());        


        //this command will show all streams
        //ffmpeg -i input.mkv 2>&1 | grep "Stream #"

        //This will be the order of the process

        //1. Sync the subtitles. This is done first because it increases the chance of success if the file has a sync subtitle in it. (Most likely English that came with the video)
        //As a result, this program has a runtime dependency of ffsubsync https://github.com/smacke/ffsubsync
        //ffs input.mkv -i input-sub.srt -o output-sub.srt

        //2. Change the video langauge to Japanese. This is because we purge all English tracks in the next command and this is sometimes listed as in English to ffmpeg
        //Again, run time dependecy of ffmpeg, but most Linux distros have it anyway. 
        //ffmpeg -i input.mpv -c copy -map 0 -metadata:s:v:0 language=jpn output.mkv
        
        //3. Remove all English tracks, and all subtitle tracks.
        //ffmpeg -i input.mkv -c copy -map 0 -map -0:m:language:eng -sn  output.mkv

        //4. Add subtitle to final video
        //ffmpeg -i input.mkv -sub_charenc 'UTF-8' -f srt -i input.srt -map 0:0 -map 0:1 -map 1:0 -c:v copy -c:a copy -c:s srt out.mkv

        //5. Remove the temp folder

        //Boom we got a synced Japanese subtitled show!

        //If demand is wanted, I can adapt this to all languages, but right now it's my personal tool I'm sharing.


        
        //trying my best to keep it os agnostic, but this will only be tested on Linux

        let mut tmp = env::temp_dir();
        tmp.push("ejani");
        fs::create_dir_all(&tmp)?;

        fs::create_dir_all(self.output_path.to_string())?;
        

        for it in anime_paths.iter().zip(sub_paths.iter()) {
            let (a_path, s_path) = it;

            let vid_file = format!(r#"{}"#, a_path.path().to_str().unwrap());
            let sub_file = format!(r#"{}"#, s_path.path().to_str().unwrap());

            let vid_name = a_path.file_name().to_str().unwrap().to_string();
            let sub_name = s_path.file_name().to_str().unwrap().to_string();

            println!("Syncing sub file {} with video {}", sub_name, vid_name);
            let __sub_sync = Command::new("ffs")
                                    .arg(vid_file.to_string())
                                    .arg("-i")
                                    .arg(sub_file.to_string())
                                    .arg("-o")
                                    .arg(format!(r#"{}/{}"#, tmp.to_str().unwrap(), sub_name))
                                    .output()
                                    .expect("Fucky wucky in the sub syncing!");


            //ffmpeg -i input.mpv -c copy -map 0 -metadata:s:v:0 language=jpn output.mkv
            println!("Converting {} video track to Japanese", vid_name);
            let __japanese_conversion = Command::
                                     new("ffmpeg")
                                    .arg("-y")
                                    .arg("-i")
                                    .arg(vid_file.to_string())
                                    .arg("-c")
                                    .arg("copy")
                                    .arg("-map")
                                    .arg("0")
                                    .arg("-metadata:s:v:0")
                                    .arg("language=jpn")
                                    .arg(format!(r#"{}/temp-1-{}"#, tmp.to_str().unwrap(), vid_name))
                                    .output()
                                    .expect("Fucky wucky in English purge!");

            //ffmpeg -i input.mkv -c copy -map 0 -map -0:m:language:eng -sn  output.mkv
            println!("Removing English tracks from {}", vid_name);
            let __english_purge = Command::
                                     new("ffmpeg")
                                    .arg("-i")
                                    .arg(format!(r#"{}/temp-1-{}"#, tmp.to_str().unwrap(), vid_name))
                                    .arg("-c")
                                    .arg("copy")
                                    .arg("-map")
                                    .arg("0")
                                    .arg("-map")
                                    .arg("-0:m:language:eng")
                                    .arg("-sn")
                                    .arg(format!(r#"{}/temp-2-{}"#, tmp.to_str().unwrap(), vid_name))
                                    .output()
                                    .expect("Fucky wucky in English purge!");
            //to save space we remove the 1st pass through of the ffmpeg
            fs::remove_file(format!(r#"{}/temp-1-{}"#, tmp.to_str().unwrap(), vid_name))?;


            //ffmpeg -i input.mkv -sub_charenc 'UTF-8' -f srt -i input.srt -map 0:0 -map 0:1 -map 1:0 -c:v copy -c:a copy -c:s srt out.mkv
            println!("Adding synced {} to {}", sub_name, vid_name);
            let __japanese_revolution = Command::
                                     new("ffmpeg")
                                    .arg("-i")
                                    .arg(format!(r#"{}/temp-2-{}"#, tmp.to_str().unwrap(), vid_name))
                                    .arg("-sub_charenc")
                                    .arg("'UTF-8'")
                                    .arg("-f")
                                    .arg("srt")
                                    .arg("-i")
                                    .arg(format!(r#"{}/{}"#, tmp.to_str().unwrap(), sub_name))
                                    .arg("-map")
                                    .arg("0:0")
                                    .arg("-map")
                                    .arg("0:1")
                                    .arg("-map")
                                    .arg("1:0")
                                    .arg("-c:v")
                                    .arg("copy")
                                    .arg("-c:a")
                                    .arg("copy")
                                    .arg("-c:s")
                                    .arg("srt")
                                    .arg(format!(r#"{}/{}"#, self.output_path, vid_name))
                                    .output()
                                    .expect("Fucky wucky in Japanese revolution!");
        }
        Ok(())
    }
}

//returns a Result, Err if the path is empty
fn pull_and_alphabetize(path: String, filter: &str) -> Result<std::vec::Vec<std::fs::DirEntry>, std::io::Error> {
    let mut sub_paths: Vec<_> = fs::read_dir(path)?
                        .into_iter()
                        .filter(|r| r.is_ok())
                        .map(|r| r.unwrap())
                        //this line is fucky, it's a horrible way to filter the path to only .mkv files
                        .filter_map(|r| r.path().to_str().and_then(|f| if f.ends_with(&filter) { Some(r) } else { None } ))
                        .collect();

    sub_paths.sort_by_key(|dir| dir.path());
    Ok(sub_paths)
}