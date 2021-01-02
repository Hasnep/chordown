use serde_derive::Deserialize;
use std::fs;
use std::str::Lines;
use std::{env, str::SplitWhitespace};
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
            .collect::<Vec<str>>()
            .join("\n")
            .as_str(),
    )
    .unwrap();

    println!("{:?}", frontmatter);

    let body_lines: Vec<str> = contents_lines
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

struct Chord {
    root: Note,
    extension: Option<str>,
    bass: Option<str>,
}

fn parse_chord(chord: str) -> Chord {
    for token in chord
        .split_terminator("")
    {
        match token.to_ascii_uppercase() {
            ""=>pass,
            "A" | "B" | "C" | "D" | "E" | "F" | "G" => Note::token,
            _ => token,
        }
    }
    return Chord {
        root: 'C',
        extension: ' ',
        bass: ' ',
    };
}

fn parse_body(body: Vec<str>) {
    for line in body {}
}

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
    chords: Vec<&str>,
    lyrics: Vec<&str>,
}
