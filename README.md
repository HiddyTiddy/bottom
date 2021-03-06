# π₯Ί

π₯Ί is an unstack-based esoteric programming language written in 2021 by [User:RocketRace](https://esolangs.org/wiki/User:RocketRace).
It is inspired by [bottom](https://github.com/kaylynn234/bottom) by [kaylynn234](https://github.com/kaylynn234).
It is a dialect of "bottom", a natural language used by bottoms.

See: [π₯Ί on esolangs.org](https://esolangs.org/wiki/%F0%9F%A5%BA)

|    Please  |  π₯Ίπ₯Ίπ₯Ί                                                                                                                                                           |
| -------    | ------                                                                                                                                                            |
|     π₯Ί`N`    | Pushes an integer `N` to the bottom of the unstack                                                                                                                |
|     π`N`    | Pops an integer from the unstack, and pushes the result of floor division of that integer by `N` to the unstack.                                                  |
|     ππ`N`  | Take the `N`th value in the unstack and swap it with the bottom value.                                                                                            |
|     π`N`    | Pops two integers from the unstack, then pops and discards `N` values from the unstack, then pushes the product of the two popped integers to the unstack         |
|     β¨`N`    | Duplicates the `N` values at the bottom of the unstack                                                                                                            |
|    π«`N`     | Pop a value from the bottom of the unstack. Jump back `N` instructions if the value is nonzero                                                                    |

## Installation

```sh
git clone https://github.com/HiddyTiddy/bottom
cd bottom
cargo test # for unstack tests
cargo install --path .
# run a program in this superior programming language
bottom ./examples/yes
```
