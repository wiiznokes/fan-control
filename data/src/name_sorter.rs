use std::cmp::Ordering;

pub fn add_sorted(sorted_vec: &mut Vec<String>, name: String) -> usize {
    let insert_position = match sorted_vec.binary_search_by(|e| compare_names(&name, e)) {
        Ok(_position) => panic!(), // Element already exists at this position
        Err(position) => position, // Element doesn't exist, insert at this position
    };
    sorted_vec.insert(insert_position, name);
    insert_position
}

pub fn compare_names(name1: &str, name2: &str) -> Ordering {
    if name1 == "None" {
        return Ordering::Less;
    }
    if name2 == "None" {
        return Ordering::Greater;
    }
    parse_to_lexemes(name1).cmp(&parse_to_lexemes(name2))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Lexeme {
    String(String),
    Number(i32),
    Special(String),
}

impl Ord for Lexeme {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Lexeme::String(str1), Lexeme::String(str2)) => {
                for (chr1, chr2) in str1.chars().zip(str2.chars()) {
                    let cmp = chr1.to_ascii_lowercase().cmp(&chr2.to_ascii_lowercase());
                    if cmp == Ordering::Equal {
                        if chr1.is_ascii_lowercase() != chr2.is_ascii_lowercase() {
                            if chr1.is_ascii_uppercase() {
                                return Ordering::Less;
                            } else {
                                return Ordering::Greater;
                            }
                        } else {
                            continue;
                        }
                    }
                    return cmp;
                }
                // should be unreachable
                Ordering::Equal
            }
            (Lexeme::Number(num1), Lexeme::Number(num2)) => num1.cmp(num2),
            (Lexeme::String(_), Lexeme::Number(_)) => std::cmp::Ordering::Greater,
            (Lexeme::Number(_), Lexeme::String(_)) => std::cmp::Ordering::Less,
            (Lexeme::String(_), Lexeme::Special(_)) => Ordering::Greater,
            (Lexeme::Number(_), Lexeme::Special(_)) => Ordering::Greater,
            (Lexeme::Special(_), Lexeme::String(_)) => Ordering::Less,
            (Lexeme::Special(_), Lexeme::Number(_)) => Ordering::Less,
            (Lexeme::Special(spe1), Lexeme::Special(spe2)) => spe1.cmp(spe2),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lexemes(Vec<Lexeme>);

impl Ord for Lexemes {
    fn cmp(&self, other: &Self) -> Ordering {
        for (lexeme1, lexeme2) in self.0.iter().zip(other.0.iter()) {
            let cmp = lexeme1.cmp(lexeme2);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        self.0.len().cmp(&other.0.len())
    }
}

fn parse_to_lexemes(name: &str) -> Lexemes {
    enum State {
        String,
        Number,
        Special,
    }

    impl From<&char> for State {
        fn from(value: &char) -> Self {
            if value.is_ascii_digit() {
                State::Number
            } else if value.is_alphabetic() {
                State::String
            } else {
                State::Special
            }
        }
    }

    let mut lexemes = Vec::new();
    let mut chars = name.chars();

    let first_char = chars.next().unwrap();

    let mut previous_state = State::from(&first_char);
    let mut letters = first_char.to_string();

    for char in chars {
        let current_state = State::from(&char);
        match previous_state {
            State::String => match current_state {
                State::String => {}
                State::Number | State::Special => {
                    lexemes.push(Lexeme::String(letters.clone()));
                    letters.clear();
                    previous_state = current_state;
                }
            },
            State::Number => match current_state {
                State::Number => {}
                State::Special | State::String => {
                    lexemes.push(Lexeme::Number(letters.parse::<i32>().unwrap()));
                    letters.clear();
                    previous_state = current_state;
                }
            },
            State::Special => match current_state {
                State::Special => {
                    lexemes.push(Lexeme::Special(letters.clone()));
                    letters.clear();
                }
                State::Number | State::String => {
                    lexemes.push(Lexeme::Special(letters.clone()));
                    letters.clear();
                    previous_state = current_state;
                }
            },
        }
        letters.push(char);
    }
    match previous_state {
        State::String => {
            lexemes.push(Lexeme::String(letters));
        }
        State::Number => {
            lexemes.push(Lexeme::Number(letters.parse::<i32>().unwrap()));
        }
        State::Special => {
            lexemes.push(Lexeme::Special(letters));
        }
    }

    Lexemes(lexemes)
}

impl PartialOrd for Lexemes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Lexeme {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
