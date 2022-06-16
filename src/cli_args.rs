use clap::Parser;

const LONG_ABOUT: &'static str = "\
Change normal text to emoji text 🎉

* Uppercase characters are converted to \":alphabet-white-<x>:\"
* Lowercase characters are converted to \":alphabet-yellow-<x>:\"
* Space emoji can be customized (see -s option)
* '!' and '?' are supported
";

#[derive(Parser, Debug)]
#[clap(long_about = LONG_ABOUT)]
pub struct Args {
    #[clap(value_parser, help = "Words to be parsed")]
    pub words: Vec<String>,
    
    // emote to use as a whitespace character
    #[clap(short, long, default_value_t = String::from(":white_small_square:"), value_parser, help="emote to use as a whitespace character")]
    pub space: String,
}
