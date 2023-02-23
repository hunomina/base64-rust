use clap::{command, Arg, ArgAction};

mod base64;

const DECODE_FLAG: &str = "decode?";
const INPUT_PARAMETER_NAME: &str = "input";
fn main() {
    let matches = command!()
        .arg(
            Arg::new(DECODE_FLAG)
                .short('d')
                .long("decode")
                .long_help("Use in order to decode the input string")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new(INPUT_PARAMETER_NAME))
        .get_matches();

    let input = matches
        .get_one::<String>(INPUT_PARAMETER_NAME)
        .expect("Input parameter missing")
        .to_owned();

    let result = match matches.get_flag(DECODE_FLAG) {
        true => base64::decode(input),
        false => base64::encode(input),
    };

    println!("{}", result);
}
