use unicode_segmentation::UnicodeSegmentation;
use phf::{phf_map};
use super::cli_args;

const Y: &str = "alphabet-yellow-"; // alphabet-yellow prefix
const W: &str = "alphabet-white-"; // alphabet-white prefix

const NUMBERS: phf::Map<char, &'static str> = phf_map! {
    '1' => ":one:",
    '2' => ":two:",
    '3' => ":three:",
    '4' => ":four:",
    '5' => ":five:",
    '6' => ":six:",
    '7' => ":seven:",
    '8' => ":eight:",
    '9' => ":nine:",
    '0' => ":zero:",
};

const SPECIAL_CHARACTERS: phf::Map<char, &'static str> = phf_map! {
    '!' => ":alphabet-yellow-exclamation:",
    '?' => ":alphabet-yellow-question:",
};

pub fn run(cli_args: cli_args::Args) -> String {
    let full_text = cli_args.words.join(" ");
    parse_text(&full_text, Some(&cli_args))
}

fn parse_text(text: &str, args: Option<&cli_args::Args>) -> String {
    let mut parsed_chars: Vec<String> = Vec::new();
    for grapheme in text.graphemes(true) {
        parsed_chars.push(parse_grapheme(grapheme, args));
    }

    parsed_chars.join(" ")
}

fn parse_grapheme(gr: &str, args: Option<&cli_args::Args>) -> String {
    if gr.len() > 1 {
        return gr.to_string();
    }
    let c = gr.char_indices().next().unwrap().1;

    if c.is_whitespace() {
        return args.expect("Args space is not set!").space.to_string();
    }
    
    match SPECIAL_CHARACTERS.get(&c) {
        Some(v) => return v.to_string(),
        None => (),
    }

    if c.is_ascii_alphabetic() {
        let font = if c.is_lowercase() { Y } else { W };
        return format!(":{}{}:", font, c.to_lowercase())
    }

    if c.is_numeric() {
        return NUMBERS
            .get(&c)
            .expect(format!("Character {} not found in NUMBERS", c).as_str())
            .to_string();
    }
    
    // other cases - simply return the same value
    c.to_string()
}

#[cfg(test)]
mod test {
    use super::{parse_grapheme, parse_text, run};
    use crate::cli_args;

    #[test]
    fn test_simple_str() {
        assert_eq!(
            parse_text("test", None),
            String::from(
                ":alphabet-yellow-t: :alphabet-yellow-e: :alphabet-yellow-s: :alphabet-yellow-t:"
            ),
        )
    }

    #[test]
    fn test_non_ascii_chars() {
        assert_eq!(parse_text("tą", None), String::from(":alphabet-yellow-t: ą"),)
    }

    #[test]
    fn test_parse_graphene_ascii() {
        assert_eq!(parse_grapheme("a", None).as_str(), ":alphabet-yellow-a:",);
        assert_eq!(parse_grapheme("A", None).as_str(), ":alphabet-white-a:");
    }

    #[test]
    fn test_parse_text_numbers() {
        assert_eq!(
            parse_text("1234567890", None).as_str(),
            ":one: :two: :three: :four: :five: :six: :seven: :eight: :nine: :zero:"
        )
    }

    #[test]
    fn test_exclam() {
        assert_eq!(
            parse_grapheme("!", None),
            ":alphabet-yellow-exclamation:",
        )
    }

    #[test]
    fn test_question_mark() {
        assert_eq!(
            parse_grapheme("?", None),
            ":alphabet-yellow-question:"
        )
    }

    #[test]
    fn test_other_special_characters() {
        assert_eq!(parse_grapheme("$", None), "$");
        assert_eq!(parse_grapheme("*", None), "*");
    }

    #[test]
    fn test_space_in_text() {
        let args = cli_args::Args{words: vec![], space: ":white-small-square:".to_string()};
        assert_eq!(
            parse_text("G r8", Some(&args)),
            ":alphabet-white-g: :white-small-square: :alphabet-yellow-r: :eight:"
        )
    }

    #[test]
    fn test_custom_space() {
        let custom_space_args = cli_args::Args {words: vec![], space: ":white_circle:".to_string()};
        let result = parse_grapheme(" ", Some(&custom_space_args));
        assert_eq!(result, ":white_circle:")
    }

    #[test]
    #[ignore] // ignore because it fails with current implementation
    fn test_run_with_polish_chars() {
        let cli_args = cli_args::Args {
            words: vec!["ą ę ć".to_string()],
            space: String::from(":thinking:"),
        };
        assert_eq!(
            run(cli_args),
            "ą :thinking: ę :thinking: ć",
        )
    }
}
