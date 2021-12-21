use sdl2::pixels::Color;
use std::env;

pub struct ProgramOptions {
    pub title: String,
    pub size: (u32, u32),
    pub scale: f32,
    pub button_sep: u32,
    pub button_size: u32,
    pub x_color: Color,
    pub o_color: Color,
    pub opponent_ai: bool,
}

impl ProgramOptions {
    pub fn get() -> Result<Self, String> {
        let mut out = Self {
            title: String::from("TicTacToe"),
            size: (3, 3),
            scale: 1.0,
            button_sep: 10,
            button_size: 200,
            x_color: Color::RGB(50, 200, 150),
            o_color: Color::RGB(255, 50, 50),
            opponent_ai: false,
        };
        let mut recording = None;
        for arg in env::args().nth(1) {
            if let Some(recording_arg) = recording {
                match recording_arg {
                    "title" => out.title = String::from(arg),
                    "width" => out.size.0 = arg.parse().or(Err(String::from("Failed to parse width arg")))?,
                    "height" => out.size.1 = arg.parse().or(Err(String::from("Failed to parse height arg")))?,
                    "scale" => out.scale = arg.parse().or(Err(String::from("Failed to parse scale arg")))?,
                    "button_sep" => out.button_sep = arg.parse().or(Err(String::from("Failed to parse button_sep arg")))?,
                    "button_size" => out.button_size = arg.parse().or(Err(String::from("Failed to parse button_size arg")))?,
                    "x_color" => {
                        let rgb = arg.parse::<u32>().or(Err(String::from("Failed to parse x_color arg")))?.to_ne_bytes();
                        out.x_color = Color::RGB(rgb[0], rgb[1], rgb[2]);
                    },
                    "o_color" => {
                        let rgb = arg.parse::<u32>().or(Err(String::from("Failed to parse y_color arg")))?.to_ne_bytes();
                        out.o_color = Color::RGB(rgb[0], rgb[1], rgb[2]);
                    },
                    "opponent" => out.opponent_ai = arg == "ai",
                    _ => {},
                }
                recording = None;
            } else {
                if &arg[..2] == "--" {
                    if &arg[2..] == "help" {
                        println!("\t\t\tTicTacToe\n
--title [string - default='TicTacToe']\t\t\t- Title on window titlebar
--width [unsigned 32-bit int - default=3]
--height [unsigned 32-bit int - default=3]
--scale [32-bit float - default=1.0]
--button_sep [unsigned 32-bit int - default=10]
--button_size [unsigned 32-bit int - default=200]
--x_color [unsigned 32-bit int - default=0x32C896]
--o_color [unsigned 32-bit int - default=0xFF3232]
--opponent [string - default=player]\t\t\t- If arg is 'ai', opponent will be ai. Otherwise, opponent will be player.");
                        return Err(String::from("Help called"));
                    }
                    recording = Some(match &arg[2..] {
                        "title" => "title",
                        "width" => "width",
                        "height" => "height",
                        "scale" => "scale",
                        "button_sep" => "button_sep",
                        "button_size" => "button_size",
                        "x_color" => "x_color",
                        "o_color" => "o_color",
                        "opponent" => "opponent",
                        &_ => "",
                    });
                }
            }
        }
        Ok(out)
    }
}


