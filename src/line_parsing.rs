use crate::chords::{parse_chord, Chord};

fn parse_line_chords(line: &str) -> Option<Vec<Chord>> {
    let mut chords = line
        .trim()
        .split_whitespace()
        .map(|chord_string| parse_chord(chord_string));
    if chords.any(|chord| chord.is_none()) {
        return None;
    } else {
        return Some(chords.map(|chord| chord.unwrap()).collect());
    }
}

fn parse_line_lyrics(line: &str) -> Vec<&str> {
    line.split('^').map(|s| s.trim()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_chords() {
        let test_cases = vec![("C", vec![parse_chord("C")])];
        for (input, expected_output) in test_cases {
            assert!(
                parse_line_chords(input).is_some(),
                "Chord line should parse currently."
            );
        }
    }

    #[test]
    fn test_parse_line_chords_fails_on_invalid_input() {
        let test_cases = vec!["Q", "C D E Q"];
        for input in test_cases {
            assert!(
                parse_line_chords(input).is_none(),
                "Invalid chord line should not parse."
            );
        }
    }
}
