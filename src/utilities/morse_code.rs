use std::fmt::Display;

pub(crate) trait ToMorseCode {
    fn to_morse_code(&self) -> Message;
}

impl ToMorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut res: Message = vec![];
        for ch in self.chars() {
            let letter = morse_letter_generator(ch);
            res.push(letter);
        }
        res
    }
}

fn morse_letter_generator(ch: char) -> Letter {
    match ch {
        'a' | 'A' => vec![Pulse::Long, Pulse::Long],
        'b' | 'B' => vec![Pulse::Short, Pulse::Short],
        _ => vec![],
    }
}

type Message = Vec<Letter>;

pub fn print_morse(message: Message) {}

// impl Display for Message {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut string = String::from("");
//         for letter in self {
//             for pulse in letter {
//                 string += pulse.to_owned();
//             }
//             string.push_str("\n")
//         }
//         write!(f, "({:?})", string)
//     }
// }

type Letter = Vec<Pulse>;

#[derive(Debug)]
pub enum Pulse {
    Short,
    Long,
}
