#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde_derive::Deserialize;
use std::fs;
use std::str::FromStr;
use std::str::Lines;
use std::env;
use toml;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let contents_lines = contents.lines();

    let (frontmatter_index, body_index) = split_frontmatter_body(contents_lines.clone());

    println!(
        "Found frontmatter between lines {} and {}",
        frontmatter_index, body_index
    );

    let frontmatter: Header = toml::from_str(
        contents_lines
            .clone()
            .skip(frontmatter_index + 1)
            .take(body_index - frontmatter_index - 1)
            .collect::<Vec<&str>>()
            .join("\n")
            .as_str(),
    )
    .unwrap();

    println!("{:?}", frontmatter);

    let body_lines: Vec<&str> = contents_lines
        .clone()
        .skip(body_index + 1)
        .take(5)
        .collect();

    println!("{:?}", body_lines);
}

fn split_frontmatter_body(lines: Lines) -> (usize, usize) {
    let separator_positions: Vec<usize> = lines
        .enumerate()
        .filter(|(_i, l)| l == &String::from("+++"))
        .map(|(i, _l)| i)
        .collect();
    if separator_positions.len() >= 2 {
        let start = separator_positions[0];
        let end = separator_positions[1];
        return (start, end);
    } else {
        return (0, 0);
    }
}

#[derive(Debug,PartialEq, Eq)]
enum Note {
    Af,
    A,
    As,
    Bf,
    B,
    Bs,
    Cf,
    C,
    Cs,
    Df,
    D,
    Ds,
    Ef,
    E,
    Es,
    Ff,
    F,
    Fs,
    Gf,
    G,
    Gs,
}

impl FromStr for Note {
    type Err = ();
    fn from_str(input: &str) -> Result<Note, Self::Err> {
        match input {
            "Ab" => Ok(Note::Af),
            "A" => Ok(Note::A),
            "A#" => Ok(Note::As),
            "Bb" => Ok(Note::Bf),
            "B" => Ok(Note::B),
            "B#" => Ok(Note::Bs),
            "Cb" => Ok(Note::Cf),
            "C" => Ok(Note::C),
            "C#" => Ok(Note::Cs),
            "Db" => Ok(Note::Df),
            "D" => Ok(Note::D),
            "D#" => Ok(Note::Ds),
            "Eb" => Ok(Note::Ef),
            "E" => Ok(Note::E),
            "E#" => Ok(Note::Es),
            "Fb" => Ok(Note::Ff),
            "F" => Ok(Note::F),
            "F#" => Ok(Note::Fs),
            "Gb" => Ok(Note::Gf),
            "G" => Ok(Note::G),
            "G#" => Ok(Note::Gs),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Chord {
    root: Note,
    extension: String,
    bass: Option<Note>,
}

// fn parse_body(body: Vec<str>) {
//     for line in body {}
// }

fn parse_line_chords(line: &str) -> Vec<&str> {
    line.trim().split_whitespace().collect()
}

fn parse_line_lyrics(line: &str) -> Vec<&str> {
    line.split('^').map(|s| s.trim()).collect()
}

// fn parse_chordown() {
//     // return Chordown;
// }

// struct Chordown {
//     header: Header,
//     body: Vec<Section>,
// }

#[derive(Deserialize, Debug)]
struct Header {
    title: String,
    subtitle: Option<String>,
    key: Option<String>,
    artist: Option<String>,
    tempo: Option<i128>,
    time: Option<String>,
    transpose: Option<i128>,
    capo: Option<i128>,
}

struct Section {
    name: String,
    repeats: i128,
    lines: Vec<Line>,
}

struct Line {
    chords: Vec<String>,
    lyrics: Vec<String>,
}

pub fn parse_chord(chord: &str) -> Chord {
    // Make sure the regex pattern is only compiled once
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"^
            (?P<root>[A-G][#b]?)
            (?P<extension>.*?)
            (?:/(?P<bass>[A-G][#b]?))?
            $"#
        )
        .unwrap();
    }
    let caps = RE.captures(chord).unwrap();
    return Chord {
        root: Note::from_str(caps.name("root").unwrap().as_str()).unwrap(),
        extension: caps.name("extension").unwrap().as_str().to_string(),
        bass: match caps.name("bass") {
            Some(v) => Note::from_str(v.as_str()).ok(),
            _ => None,
        },
    };
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_chord() {
        let test_cases = vec![
            (
                "C",
                Chord {
                    root: Note::C,
                    extension: "".to_string(),
                    bass: None,
                },
            ),
            (
                "C#",
                Chord {
                    root: Note::Cs,
                    extension: "".to_string(),
                    bass: None,
                },
            ),
            (
                "D7",
                Chord {
                    root: Note::D,
                    extension: "7".to_string(),
                    bass: None,
                },
            ),
            (
                "Eb/C",
                Chord {
                    root: Note::Ef,
                    extension: "".to_string(),
                    bass: Some(Note::C),
                },
            ),
            (
                "Emaj7/C#",
                Chord {
                    root: Note::E,
                    extension: "maj7".to_string(),
                    bass: Some(Note::Cs),
                },
            ),
        ];
        for (input, expected_output) in test_cases {
            assert_eq!(parse_chord(input).root, expected_output.root);
            assert_eq!(parse_chord(input).extension, expected_output.extension);
            assert_eq!(parse_chord(input).bass, expected_output.bass);
        }
    }
}
