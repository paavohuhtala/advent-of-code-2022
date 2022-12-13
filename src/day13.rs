use std::{cmp::Ordering, collections::VecDeque};

use itertools::{EitherOrBoth, Itertools};

const INPUT: &str = include_str!("./day13.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    LBracket,
    RBracket,
    Comma,
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Number(i64),
    List(Vec<Value>),
}

type Tokens = VecDeque<Token>;

fn tokenize_line(line: &str) -> Tokens {
    let mut tokens = VecDeque::new();
    let chars = line.chars().collect::<Vec<_>>();
    let mut char_iter = chars.iter().copied().peekable();

    while let Some(token) = char_iter.next() {
        match token {
            '[' => tokens.push_back(Token::LBracket),
            ']' => tokens.push_back(Token::RBracket),
            ',' => tokens.push_back(Token::Comma),
            '0'..='9' => {
                let mut number = token.to_digit(10).unwrap() as i64;
                while let Some(&next) = char_iter.peek() {
                    if next.is_digit(10) {
                        number = number * 10 + next.to_digit(10).unwrap() as i64;
                        char_iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push_back(Token::Number(number));
            }
            _ => panic!("Unexpected token: {}", token),
        }
    }

    tokens
}

fn pop_and_expect(tokens: &mut Tokens, expected: Token) {
    let token = tokens.pop_front().unwrap();
    assert_eq!(token, expected, "Expected {:?}, got {:?}", expected, token);
}

fn parse_list(tokens: &mut Tokens) -> Value {
    pop_and_expect(tokens, Token::LBracket);

    let mut list = Vec::new();

    loop {
        match tokens.front().copied() {
            None => {
                break;
            }
            Some(Token::RBracket) => {
                pop_and_expect(tokens, Token::RBracket);
                break;
            }
            Some(Token::Comma) => {
                pop_and_expect(tokens, Token::Comma);
            }
            Some(Token::LBracket) => {
                list.push(parse_list(tokens));
            }
            Some(Token::Number(number)) => {
                pop_and_expect(tokens, Token::Number(number));
                list.push(Value::Number(number));
            }
        }
    }

    Value::List(list)
}

fn tokenize_and_parse(line: &str) -> Value {
    let mut tokens = tokenize_line(line);
    parse_list(&mut tokens)
}

fn compare_values(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.cmp(right),
        (Value::List(left), Value::List(right)) => {
            for pair in left.iter().zip_longest(right.iter()) {
                match pair {
                    EitherOrBoth::Both(l, r) => match compare_values(l, r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    },
                    EitherOrBoth::Left(_) => return Ordering::Greater,
                    EitherOrBoth::Right(_) => return Ordering::Less,
                };
            }

            Ordering::Equal
        }
        (Value::Number(left), right @ Value::List(_)) => {
            let left = Value::List(vec![Value::Number(*left)]);
            compare_values(&left, right)
        }
        (left @ Value::List(_), Value::Number(right)) => {
            let right = Value::List(vec![Value::Number(*right)]);
            compare_values(left, &right)
        }
    }
}

pub fn day13a() {
    let input = INPUT
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(tokenize_and_parse)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let result = input
        .into_iter()
        .enumerate()
        .map(|(i, (left, right))| {
            if compare_values(&left, &right) == Ordering::Less {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Day13a: {}", result);
}

pub fn day13b() {
    let mut input = INPUT
        .lines()
        .filter(|line| line.len() > 0)
        .map(tokenize_and_parse)
        .collect_vec();

    let marker1 = tokenize_and_parse("[[2]]");
    let marker2 = tokenize_and_parse("[[6]]");

    input.push(marker1.clone());
    input.push(marker2.clone());

    input.sort_by(compare_values);

    let marker1_pos = input.iter().position(|x| x == &marker1).unwrap() + 1;
    let marker2_pos = input.iter().position(|x| x == &marker2).unwrap() + 1;

    let decoder_key = marker1_pos * marker2_pos;

    println!("Day13b: {}", decoder_key);
}

#[allow(dead_code)]
fn stringify_list(list: &Value) -> String {
    match list {
        Value::Number(number) => number.to_string(),
        Value::List(list) => {
            let mut string = String::new();
            string.push('[');
            for (i, item) in list.iter().enumerate() {
                if i > 0 {
                    string.push(',');
                }
                string.push_str(&stringify_list(item));
            }
            string.push(']');
            string
        }
    }
}
