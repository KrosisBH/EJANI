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

        //gathering paths. What this statement does is remove all the paths that might error (idk why but better safe than sorry)
        //then it filters all the files that don't end in .mkv (I only support this right now)
        //and then collect all those files
        let mut anime_paths: Vec<_> = fs::read_dir(self.path_to_anime)?
                                                                .into_iter()
                                                                .filter(|r| r.is_ok())
                                                                .map(|r| r.unwrap())
                                                                //this line is fucky, it's a horrible way to filter the path to only .mkv files
                                                                .filter_map(|r| r.path().to_str().and_then(|f| if f.ends_with(".mkv") { Some(r) } else { None } ))
                                                                .collect();
        //alphabetizes the path
        anime_paths.sort_by_key(|dir| dir.path());


        //see above, but I filter all but .srt and .ass (haha ass)
        let mut sub_paths: Vec<_> = fs::read_dir(self.path_to_subs)?
                                                                .into_iter()
                                                                .filter(|r| r.is_ok())
                                                                .map(|r| r.unwrap())
                                                                //this line is fucky, it's a horrible way to filter the path to only .mkv files
                                                                .filter_map(|r| r.path().to_str().and_then(|f| if f.ends_with(".srt") || f.ends_with(".ass") { Some(r) } else { None } ))
                                                                .collect();

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
        

        for it in anime_paths.iter().zip(sub_paths.iter()) {
            let (a_path, s_path) = it;

            let vid_file = format!(r#"{}"#, a_path.path().to_str().unwrap());
            let sub_file = format!(r#"{}"#, s_path.path().to_str().unwrap());

            let vid_name = a_path.file_name().to_str().unwrap().to_string();
            let sub_name = s_path.file_name().to_str().unwrap().to_string();

            //subs are first
            println!("Syncing sub file {} with video {}", sub_name, vid_name);
            let sub_sync = Command::new("ffs")
                                    .arg(vid_file.to_string())
                                    .arg("-i")
                                    .arg(sub_file.to_string())
                                    .arg("-o")
                                    .arg(format!(r#"{}/{}"#, tmp.to_str().unwrap(), sub_name))
                                    .output()
                                    .expect("Fucky wucky in the sub syncing!");


            //ffmpeg -i input.mpv -c copy -map 0 -metadata:s:v:0 language=jpn output.mkv
            println!("Converting {} video track to Japanese", vid_name);
            let japanese_conversion = Command::new("ffmpeg")
                                    .arg("-y")
                                    .arg("-i")
                                    .arg(vid_file.to_string())
                                    .arg("-c")
                                    .arg("copy")
                                    .arg("-map")
                                    .arg("0")
                                    .arg("-metadata:s:v:0")
                                    .arg("language=jpn")
                                    .arg(format!(r#"{}/1-{}"#, tmp.to_str().unwrap(), vid_name))
                                    .output()
                                    .expect("Fucky wucky in English purge!");

            //ffmpeg -i input.mkv -c copy -map 0 -map -0:m:language:eng -sn  output.mkv
            println!("Removing English tracks from {}", vid_name);
            let english_purge = Command::new("ffmpeg")
                                    .arg("-y")
                                    .arg("-i")
                                    .arg(vid_file.to_string())
                                    .arg("-c")
                                    .arg("copy")
                                    .arg("-map")
                                    .arg("0")
                                    .arg("-map")
                                    .arg("-0:m:language:eng")
                                    .arg("-sn")
                                    .arg(format!(r#"{}/2-{}"#, tmp.to_str().unwrap(), vid_name));
                                    //.output()
                                    //.expect("Fucky wucky in English purge!");

            //println!("{}", String::from_utf8(english_purge.stderr).unwrap());

        }



        //fs::remove_dir_all(&tmp)?;

        Ok(())
    }
}