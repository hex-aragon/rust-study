const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character))
    {
        panic!("Please only input numbers, +-, or spaces");
    }

    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|character| *character != ' ')
        .collect::<String>();
    println!("{input}");
    let mut result_vec = vec![];
    let mut push_string = String::new();
    for character in input.chars() {
        match character {
            '+' => {
                if !push_string.is_empty() {
                    // "" 안 넣으려고
                    result_vec.push(push_string.clone());
                    push_string.clear(); // 빈 string
                }
            }
            '-' => {
                if push_string.contains('-') || push_string.is_empty() {
                    //아무것도 안 들어 있고 또는 다른 마이너스가 있을 경우 push_string
                    push_string.push(character);
                } else {
                    //숫자인 경우
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(character);
                }
            }
            number => {
                if push_string.contains('-') {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(number);
                } else {
                    push_string.push(number);
                }
            }
        }
    }

    result_vec.push(push_string);
    let mut total = 0;
    let mut adds = true;
    let mut math_iter = result_vec.into_iter();

    while let Some(entry) = math_iter.next() {
        if entry.contains('-') {
            // -, --, ---
            if entry.chars().count() % 2 == 1 {
                adds = match adds {
                    true => false,
                    false => true,
                };
                continue;
            } else {
                continue;
            }
        }
        if adds == true {
            total += entry.parse::<i32>().unwrap();
        } else {
            total -= entry.parse::<i32>().unwrap();
        }
    }
    total
}

fn main() {
    let my_number = math("7 + 9 + 10  ++++++++");
}

#[cfg(test)]
mod tests {
    use super::math; // super = the space just above

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }

    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - - 1"), 2);
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + please add seven");
    }
}
