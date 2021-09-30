
use std::env;
use ejani::EjaniInstance;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut ani_path = "";
    let mut sub_path = "";
    let mut out_path = "";

    match args.len() {
        1 => {
            help();
        },

        2 => {
            help();
        },

        _ => {
            for (i, argument) in args.iter().enumerate() {

                match &argument[..] {

                    "-ia" => {
                        ani_path = &args[i+1]
                    }

                    "-is" => {
                        sub_path = &args[i+1]
                    }

                    "-o" => {
                        out_path = &args[i+1]
                    }

                    _ => {
                        //hacky way to make sure the paths don't terminate the program
                        if (argument != ani_path && argument != sub_path && argument != out_path) && i>1 {
                            println!("Unknown argument {}", argument);
                            std::process::exit(1);
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
    
    if ani_path.is_empty() {
        println!("Video path wasn't passed. To pass it, use -ia <path_to_video_folder>");
        std::process::exit(1);
    }

    if sub_path.is_empty() {
        println!("Subtitle path wasn't passed. To pass it, use -is <path_to_subtitles_folder>");
        std::process::exit(1);
    }

    if out_path.is_empty() {
        println!("Output path wasn't passed. To pass it, use -o <path_to_output_folder>");
        std::process::exit(1);
    }


    let e = EjaniInstance::new
    (
        ani_path.to_string(), 
        sub_path.to_string(), 
        out_path.to_string()
    );
    
    e.run().unwrap();
    println!("できた！")
}

fn help() {
    println! (
"Ejani: written by krosisbh
Usage: ejani -ia <path_to_video_folder> -is <path_to_subtitles> -o <path_to_output>

More features to be planned!

"
    );
}