use std::env;

fn main() {
    //let arguments = env::args.collect();
    let args: Vec<String> = env::args().collect();
    println!("{}", capitalize(&args[1]));
}

fn capitalize(s: &str) -> String {
    let mut res = "".to_string();
    let mut prec_char = ' ';
    for c in s.chars() {
        if prec_char == ' ' && c.is_alphabetic() {
            res.extend(c.to_uppercase().to_string().chars());
        } else {
            res.push(c);
        }
        prec_char = c;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_word() {
        let test = super::capitalize("ciao");
        assert_eq!(test, "Ciao", "When input string is composed by only one word, without spaces, capitalize function fail");
    }

    #[test]
    fn more_then_one_word() {
        let test = super::capitalize("ciao caro, come stai?");
        assert_eq!(test, "Ciao Caro, Come Stai?", "When input string is composed by more than one word, capitalize function fail");
    }

    #[test]
    fn accent() {
        let test = super::capitalize("lo sai che è sempre bello vederti");
        assert_eq!(test, "Lo Sai Che È Sempre Bello Vederti", "When input string has a word staring with accent, capitalize function fail");
    }

    #[test]
    fn empty() {
        let test = super::capitalize("");
        assert_eq!(test, "", "When input string has a word staring with accent, capitalize function fail");
    }

    #[test]
    fn more_than_one_spaces() {
        let test = super::capitalize("ciao     ciao");
        assert_eq!(test, "Ciao     Ciao", "When input string has more than one consecutive space, capitalize function fail");
    }
}
