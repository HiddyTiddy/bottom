/*!
 * # 🥺
 *
 * 🥺 is an unstack-based esoteric programming language written in 2021 by [User:RocketRace](https://esolangs.org/wiki/User:RocketRace).
 * It is inspired by [bottom](https://github.com/kaylynn234/bottom) by [kaylynn234](https://github.com/kaylynn234).
 * It is a dialect of "bottom", a natural language used by bottoms.
 *
 * See: [🥺 on esolangs.org](https://esolangs.org/wiki/%F0%9F%A5%BA)
 *
 * |    Please  |  🥺🥺🥺                                                                                                                                                           |
 * | -------    | ------                                                                                                                                                            |
 * |     🥺N    | Pushes an integer `N` to the bottom of the unstack                                                                                                                |
 * |     💖N    | Pops an integer from the unstack, and pushes the result of floor division of that integer by `N` to the unstack.                                                  |
 * |     👉👈N  | Take the `N`th value in the unstack and swap it with the bottom value.                                                                                            |
 * |     💓N    | Pops two integers from the unstack, then pops and discards `N` values from the unstack, then pushes the product of the two popped integers to the unstack         |
 * |     ✨N    | Duplicates the `N` values at the bottom of the unstack                                                                                                            |
 * |    🫂N     | Pop a value from the bottom of the unstack. Jump back `N` instructions if the value is nonzero                                                                    |
 *
 *
 */

use std::fs;

use clap::{App, Arg};

// 🥺

/**
 * # Possible Operations
 *
 * enum that represents an action and the argument associated
 *
 * would allow for remixing the commands associated with an operation
 */
#[derive(Debug, Clone, Copy)]
enum Operations {
    /// 🥺      :
    /// Pushes an integer N to the bottom of the unstack.
    Push(i64),
    /// 💖      :
    /// Pops an integer from the unstack, and pushes the result of floor division of that integer by N to the unstack.
    Pop(i64),
    /// 👉👈    :
    /// Take the Nth value in the unstack and swap it with the bottom value.
    Swap(usize),
    /// 💓      :
    /// Pops two integers from the unstack, then pops and discards N values from the unstack, then pushes the product of the two popped integers to the unstack.
    Heart(usize),
    /// ✨      :
    /// Duplicates the N values at the bottom of the unstack.
    Dup(usize),
    /// 🫂      :
    /// Pop a value from the bottom of the unstack. Jump back N instructions if the value is nonzero.
    Hug(usize),
}

mod unstack {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    struct UnstackNode {
        prev: Option<Box<UnstackNode>>,
        value: i64,
    }

    /**
     * # Unstack
     *
     * it's like a stack but you push to the bottom
     *
     * this is a linked list implementation of an Unstack
     *
     * ## Usage
     *
     * ```rust
     * let unstack: Unstack = Unstack::new();
     *
     * unstack.push(10);
     * unstack.push(42);
     *
     * assert_eq!(unstack.pop(), 42);
     * assert_eq!(unstack.pop(), 10);
     * assert!(unstack.is_empty());
     * ```
     */
    pub struct Unstack {
        bottom: Option<UnstackNode>,
        size: usize,
    }

    impl Debug for Unstack {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            if self.size == 0 {
                write!(f, "[]")
            } else {
                let mut node = self.bottom.clone().unwrap();
                let mut tmp = vec![node.value];

                while let Some(prev) = node.prev {
                    node = *prev;
                    tmp.push(node.value);
                }
                let mut builder = "[ ".to_string();
                for (i, j) in tmp.iter().rev().enumerate() {
                    builder += &*format!("{}", j);
                    if i != tmp.len() - 1 {
                        builder += ", "
                    }
                }
                write!(f, "{} ] ", builder)
            }
        }
    }

    impl Unstack {
        /// create an empty Unstack
        pub fn new() -> Self {
            Unstack {
                bottom: None,
                size: 0,
            }
        }

        /// push new value to the bottom of the unstack
        pub fn push(&mut self, value: i64) {
            if self.size == 0 {
                self.bottom = Some(UnstackNode { prev: None, value });
            } else {
                let newbottom = UnstackNode {
                    prev: Some(Box::new(self.bottom.clone().unwrap())),
                    value,
                };
                self.bottom = Some(newbottom);
            }
            self.size += 1;
        }

        /// pop a value off the bottom of the unstack and return it
        pub fn pop(&mut self) -> i64 {
            if self.size == 0 {
                panic!("out of bounds");
            }
            let bottom = self.bottom.clone().unwrap();
            self.bottom = bottom.prev.map(|prev| *prev);
            self.size -= 1;
            bottom.value
        }

        /// swaps the bottom of the unstack with the provided index
        /// note that since unstacks do not support indexing this will run in O(steps)
        pub fn swap_first(&mut self, steps: usize) {
            let mut tmp = Unstack::new();
            let bottom_val = self.bottom.clone().unwrap().value;
            for _ in 0..steps {
                tmp.push(self.pop());
            }
            let top_val = self.pop();
            self.push(bottom_val);
            for _ in 0..steps - 1 {
                self.push(tmp.pop());
            }
            self.push(top_val);
        }

        /// returns the size of the unstack
        pub fn len(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::unstack::Unstack;

        #[test]
        fn test_push() {
            let mut unstack: Unstack = Unstack::new();
            assert_eq!(unstack.size, 0);
            assert!(unstack.is_empty());
            assert!(unstack.bottom.is_none());

            let to_test = vec![420, 69, 42069, -1, -1948];
            for (size, i) in to_test.iter().enumerate() {
                unstack.push(*i);
                assert!(unstack.bottom.is_some());
                assert_eq!(unstack.bottom.clone().unwrap().value, *i);
                assert_eq!(unstack.len(), size + 1);
            }

            assert_eq!(unstack.bottom.unwrap().prev.unwrap().value, -1);
        }

        #[test]
        fn test_pop() {
            let mut unstack = Unstack::new();
            assert!(unstack.is_empty());
            assert!(unstack.bottom.is_none());

            unstack.push(1);
            unstack.push(2);
            unstack.push(3);
            unstack.push(4);

            unstack.pop();
            assert!(unstack.bottom.is_some());
            assert_eq!(unstack.bottom.as_ref().unwrap().value, 3);
            unstack.pop();
            assert!(unstack.bottom.is_some());
            assert_eq!(unstack.bottom.as_ref().unwrap().value, 2);

            assert!(!unstack.is_empty());
        }
    }
}

use crate::unstack::Unstack;

/// tokenize a 🥺 program
fn parse(source: &str) -> Vec<Operations> {
    let mut tokens = vec![];

    let mut word: String = "".to_string();
    let mut operation: Option<String> = None;
    for ch in source.chars() {
        if !(matches!(ch, '🥺' | '💖' | '👉' | '👈' | '💓' | '✨' | '🫂') || ch.is_ascii_digit())
        {
            if let Some(op) = &operation {
                let value: i64 = if let Ok(num) = word.parse() {
                    num
                } else {
                    word.chars().count() as i64
                };
                tokens.push(match op.as_str() {
                    "🥺" => Operations::Push(value),
                    "💖" => Operations::Pop(value),
                    "👉👈" => Operations::Swap(value as usize),
                    "💓" => Operations::Heart(value as usize),
                    "✨" => Operations::Dup(value as usize),
                    "🫂" => Operations::Hug(value as usize),
                    _ => unreachable!(),
                });
            }
            word = "".to_string();
            operation = None;
            continue;
        }
        word += ch.to_string().as_ref();

        if matches!(operation, None) && matches!(&*word, "🥺" | "💖" | "👉👈" | "💓" | "✨" | "🫂")
        {
            operation = Some(word);
            word = "".to_string();
        }
    }
    if word != "" {
        if let Some(op) = &operation {
            let value: i64 = if let Ok(num) = word.parse() {
                num
            } else {
                word.chars().count() as i64
            };
            tokens.push(match op.as_str() {
                "🥺" => Operations::Push(value),
                "💖" => Operations::Pop(value),
                "👉👈" => Operations::Swap(value as usize),
                "💓" => Operations::Heart(value as usize),
                "✨" => Operations::Dup(value as usize),
                "🫂" => Operations::Hug(value as usize),
                _ => unreachable!(),
            });
        }
    }
    tokens
}

/// interpret the tokens of a 🥺  program
fn interpret(tokens: Vec<Operations>) -> Vec<i64> {
    let mut unstack = Unstack::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < tokens.len() {
        match tokens[instruction_pointer] {
            Operations::Push(val) => unstack.push(val),
            Operations::Pop(val) => {
                if unstack.is_empty() {
                    panic!("💖 : empty unstack at {}", instruction_pointer);
                }
                let value = unstack.pop() / val;
                unstack.push(value);
            }
            Operations::Swap(steps) => {
                if unstack.len() < steps {
                    panic!(
                        "👉👈 : unstack too small (expected at least {}, had {}) at {}",
                        steps,
                        unstack.len(),
                        instruction_pointer
                    );
                }
                unstack.swap_first(steps)
            }
            Operations::Heart(val) => {
                if unstack.len() < 2 + val {
                    panic!(
                        "💓 : unstack too small (expected at least {}, had {}) at {}",
                        val + 2,
                        unstack.len(),
                        instruction_pointer
                    )
                }
                let value = unstack.pop() * unstack.pop();
                for _ in 0..val {
                    unstack.pop();
                }
                unstack.push(value);
            }
            Operations::Dup(val) => {
                if unstack.len() < val {
                    panic!(
                        "✨ : unstack too small (expected at least {}, had {}) at {}",
                        val,
                        unstack.len(),
                        instruction_pointer
                    )
                }
                let mut tmp = Unstack::new();
                for _ in 0..val {
                    tmp.push(unstack.pop());
                }
                for _ in 0..val {
                    let value = tmp.pop();
                    unstack.push(value);
                    unstack.push(value);
                }
            }
            Operations::Hug(val) => {
                if unstack.is_empty() {
                    panic!("🫂 : empty unstack at {}", instruction_pointer);
                }
                if unstack.pop() != 0 {
                    instruction_pointer -= val - 1;
                }
            }
        }
        instruction_pointer += 1;
    }
    let mut out = vec![];
    for _ in 0..unstack.len() {
        out.push(unstack.pop());
    }
    out
}

fn main() {
    let args = App::new("🥺 interpreter")
        .version("0.1.0")
        .author("hyde <hiddy.tiddey@gmail.com>")
        .about("see🥺 https://esolangs.org/wiki/%F0%9F%A5%BA for documentation")
        .arg(
            Arg::with_name("filename")
                .takes_value(true)
                .required(true)
                .value_name("FILE")
                .help("name of the file 🥺🥺🥺👉👈🥺")
                .index(1),
        )
        .arg(
            Arg::with_name("a")
                .short("a")
                .multiple(true)
                .help("display output as ascii"),
        )
        .get_matches();
    let filename = args.value_of("filename").expect("missing filename");
    let source = fs::read_to_string(filename).expect("could not read file");
    let tokens = parse(source.as_str());
    let output = interpret(tokens);
    if args.occurrences_of("a") == 0 {
        println!("{:?}", output);
    } else {
        for i in output {
            print!("{}", (i & 0xff) as u8 as char)
        }
        println!();
    }
}
