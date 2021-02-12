use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
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

pub fn parse_chord(chord: &str) -> Option<Chord> {
    // Make sure the regex pattern is only compiled once
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"^(?P<root>[A-G][#b]?)(?P<extension>.*?)(?:/(?P<bass>[A-G][#b]?))?$").unwrap()
    });
    let caps = match RE.captures(chord) {
        Some(c) => c,
        None => return None,
    };
    let chord = Chord {
        root: Note::from_str(caps.name("root").unwrap().as_str()).unwrap(),
        extension: caps.name("extension").unwrap().as_str().to_string(),
        bass: match caps.name("bass") {
            Some(v) => Note::from_str(v.as_str()).ok(),
            _ => None,
        },
    };
    return Some(chord);
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
            assert!(
                parse_chord(input).is_some(),
                "Chord should parse currently."
            );
            assert_eq!(parse_chord(input).unwrap().root, expected_output.root);
            assert_eq!(
                parse_chord(input).unwrap().extension,
                expected_output.extension
            );
            assert_eq!(parse_chord(input).unwrap().bass, expected_output.bass);
        }
    }

    #[test]
    fn test_fails_to_parse_invalid_chords() {
        let test_cases = vec!["", " ", "abc", "xyz", "XYZ", "123"];
        for input in test_cases {
            assert!(
                parse_chord(input).is_none(),
                "Invalid chord should not parse."
            )
        }
    }
}
