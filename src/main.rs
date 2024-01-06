mod fns;
use clap::Parser;
use colored::Colorize;
use inline_colorization::*;
use std::collections::HashMap;
use term_size;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f')]
    flag: Option<String>,

    #[arg(short = 'l')]
    list: bool,
}

fn get_flag<'a>(
    flag_name: &'a Option<String>,
    flags: &'a HashMap<&'a str, Vec<Vec<u8>>>,
) -> Option<&'a Vec<Vec<u8>>> {
    flag_name
        .as_deref()
        .and_then(|flag_name_str| flags.get(flag_name_str))
}

fn list_flags<K, V>(hashmap: &HashMap<K, V>)
where
    K: std::fmt::Debug,
{
    let keys_str: String = hashmap
        .keys()
        .map(|key| format!("{:?}", key))
        .collect::<Vec<_>>()
        .join(", ");
    print!("{}", keys_str);
}

fn main() {
    let flags: HashMap<&str, Vec<Vec<u8>>> = [
        //to add your own flag, create new vector with vectors of rgb values of every stripe
        //if flag has 3 colors - double them, so the flag would look better
        //
        //this looks so bad but it works
        (
            "gay",
            vec![
                vec![7, 142, 112],
                vec![38, 206, 170],
                vec![152, 232, 193],
                vec![240, 239, 255],
                vec![123, 173, 226],
                vec![80, 73, 203],
                vec![61, 26, 120],
            ],
        ),
        (
            "demiboy",
            vec![
                vec![127, 127, 127],
                vec![196, 196, 196],
                vec![154, 217, 235],
                vec![255, 255, 255],
                vec![154, 217, 235],
                vec![196, 196, 196],
                vec![127, 127, 127],
            ],
        ),
        (
            "abrogender",
            vec![
                vec![103, 249, 248],
                vec![173, 226, 174],
                vec![248, 246, 249],
                vec![135, 42, 221],
                vec![0, 0, 0],
            ],
        ),
        (
            "abroromantic",
            vec![
                vec![64, 191, 141],
                vec![151, 151, 151],
                vec![182, 103, 139],
                vec![214, 42, 115],
                vec![255, 0, 129],
            ],
        ),
        (
            "abrosexual",
            vec![
                vec![117, 202, 145],
                vec![179, 228, 199],
                vec![255, 255, 255],
                vec![230, 149, 181],
                vec![217, 68, 108],
            ],
        ),
        (
            "aceflux",
            vec![
                vec![199, 34, 82],
                vec![193, 39, 121],
                vec![192, 39, 154],
                vec![168, 40, 172],
                vec![140, 38, 175],
            ],
        ),
        (
            "agender",
            vec![
                vec![0, 0, 0],
                vec![188, 196, 198],
                vec![255, 255, 255],
                vec![182, 245, 131],
                vec![255, 255, 255],
                vec![188, 196, 198],
                vec![0, 0, 0],
            ],
        ),
        (
            "aroace",
            vec![
                vec![226, 140, 0],
                vec![236, 205, 0],
                vec![255, 255, 255],
                vec![98, 174, 220],
                vec![32, 56, 86],
            ],
        ),
        (
            "aromantic",
            vec![
                vec![61, 165, 66],
                vec![167, 211, 121],
                vec![255, 255, 255],
                vec![169, 169, 169],
                vec![0, 0, 0],
            ],
        ),
        (
            "asexual",
            vec![
                vec![0, 0, 0],
                vec![163, 163, 163],
                vec![255, 255, 255],
                vec![128, 0, 128],
            ],
        ),
        (
            "bigender",
            vec![
                vec![105, 147, 216],
                vec![144, 195, 234],
                vec![255, 255, 255],
                vec![196, 164, 235],
                vec![255, 255, 255],
                vec![248, 171, 197],
                vec![237, 123, 168],
            ],
        ),
        (
            "bisexual",
            vec![
                vec![214, 2, 112],
                vec![214, 2, 112],
                vec![155, 79, 150],
                vec![0, 56, 168],
                vec![0, 56, 168],
            ],
        ),
        (
            "cinthean",
            vec![
                vec![13, 45, 104],
                vec![103, 29, 166],
                vec![231, 7, 76],
                vec![236, 98, 134],
                vec![249, 158, 137],
                vec![255, 255, 255],
            ],
        ),
        (
            "demigender",
            vec![
                vec![127, 127, 127],
                vec![196, 196, 196],
                vec![251, 255, 116],
                vec![255, 255, 255],
                vec![251, 255, 116],
                vec![196, 196, 196],
                vec![127, 127, 127],
            ],
        ),
        (
            "demigirl",
            vec![
                vec![127, 127, 127],
                vec![196, 196, 196],
                vec![255, 174, 201],
                vec![255, 255, 255],
                vec![255, 174, 201],
                vec![196, 196, 196],
                vec![127, 127, 127],
            ],
        ),
        (
            "femboy",
            vec![
                vec![209, 97, 164],
                vec![228, 174, 205],
                vec![254, 254, 254],
                vec![86, 206, 247],
                vec![254, 254, 254],
                vec![228, 174, 205],
                vec![209, 97, 164],
            ],
        ),
        (
            "genderfluid",
            vec![
                vec![255, 117, 162],
                vec![255, 255, 255],
                vec![190, 24, 214],
                vec![0, 0, 0],
                vec![51, 62, 189],
            ],
        ),
        (
            "genderqueer",
            vec![
                vec![181, 126, 220],
                vec![181, 126, 220],
                vec![255, 255, 255],
                vec![255, 255, 255],
                vec![74, 129, 35],
                vec![74, 129, 35],
            ],
        ),
        (
            "lesbian",
            vec![
                vec![213, 45, 0],
                vec![255, 154, 86],
                vec![255, 255, 255],
                vec![211, 98, 164],
                vec![163, 2, 98],
            ],
        ),
        (
            "nonbinary",
            vec![
                vec![252, 244, 52],
                vec![252, 252, 252],
                vec![156, 92, 212],
                vec![44, 44, 44],
            ],
        ),
        (
            "omnisexual",
            vec![
                vec![255, 156, 205],
                vec![255, 85, 190],
                vec![36, 0, 69],
                vec![103, 95, 254],
                vec![141, 165, 254],
            ],
        ),
        (
            "pansexual",
            vec![
                vec![255, 33, 140],
                vec![255, 33, 140],
                vec![255, 216, 0],
                vec![255, 216, 0],
                vec![33, 177, 255],
                vec![33, 177, 255],
            ],
        ),
        (
            "polysexual",
            vec![
                vec![246, 28, 185],
                vec![246, 28, 185],
                vec![7, 213, 105],
                vec![7, 213, 105],
                vec![28, 146, 246],
                vec![28, 146, 246],
            ],
        ),
        (
            "pride",
            vec![
                vec![228, 3, 3],
                vec![255, 140, 0],
                vec![255, 237, 0],
                vec![0, 128, 38],
                vec![0, 77, 255],
                vec![117, 7, 135],
            ],
        ),
        (
            "transfeminine",
            vec![
                vec![116, 223, 255],
                vec![254, 225, 237],
                vec![255, 181, 215],
                vec![254, 141, 191],
                vec![255, 181, 215],
                vec![254, 225, 237],
                vec![116, 223, 255],
            ],
        ),
        (
            "transgender",
            vec![
                vec![91, 206, 250],
                vec![245, 169, 184],
                vec![255, 255, 255],
                vec![245, 169, 184],
                vec![91, 206, 250],
            ],
        ),
        (
            "transmasculine",
            vec![
                vec![255, 138, 189],
                vec![205, 245, 254],
                vec![154, 235, 255],
                vec![116, 223, 255],
                vec![154, 235, 255],
                vec![205, 245, 254],
                vec![255, 138, 189],
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let args = Cli::parse();
    let user = format!("{color_cyan}{}@{}{color_reset}", fns::get_user().unwrap_or_default(), fns::get_host().unwrap_or_default());
    let os = format!("{color_cyan}os{color_reset}: {}", fns::get_os().unwrap_or_default());
    let kernel = format!("{color_cyan}kernel{color_reset}: {}", fns::get_kernel().unwrap_or_default());
    let uptime = format!("{color_cyan}uptime{color_reset}: {}", fns::get_uptime().unwrap_or_default());
    let wm = format!("{color_cyan}wm{color_reset}: {}", fns::get_wm().unwrap_or_default());
    let shell = format!("{color_cyan}shell{color_reset}: {}", fns::get_shell().unwrap_or_default());
    let terminal = format!("{color_cyan}terminal{color_reset}: {}", fns::get_terminal().unwrap_or_default());
    let line: String;
    let data: [String; 7] = [user, os, kernel, uptime, wm, shell, terminal];
    let mut num = 0;
    if args.flag.is_some() {
        if let Some((termw, _termh)) = term_size::dimensions() {
            line = " ".repeat((termw as f32 * 0.2) as usize);
        } else {
            eprintln!("Unable to get terminal size");
            return;
        }
        match get_flag(&args.flag, &flags) {
            Some(flag) => {
                for color in flag {
                    println!("{}    {}", line.on_truecolor(color[0] as u8, color[1] as u8, color[2] as u8), data[num]);
                    num += 1; //i literally couldnt think of anything better than += 1
                }
                while num < data.len() {
                    println!("{}    {}", line, data[num]);
                    num += 1;
                }
            }
            None => {
                println!("Flag not found: {}", args.flag.unwrap_or(String::from("None")));
            }
        }
    } else if args.list {
        list_flags(&flags);
    }
}
