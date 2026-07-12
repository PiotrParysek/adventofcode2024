# Advent of Code 2024 

[Advent of Code 2024](https://adventofcode.com/2024)

## Structure

This is a template for solving Advent of Code puzzles in Rust.

It's structured as a single Cargo crate with a library and a binary.

- `src/lib.rs`: Contains the main solve function that dispatches to the correct day's module. It also declares all day modules.
- `src/main.rs`: A simple command-line runner that parses args (day, part) and calls solve.
- `src/common.rs`: For any shared helper functions (like reading input).
- `src/dayXX.rs`: Each day gets its own module file.
- `input/`: This directory holds your puzzle inputs.
- `tasks/`: This directory holds your puzzle description

### TASKS

Table for `README` for each task and if the task is completed

| DAY | TASK 1 | TASK 2 |
| :---: | :---: | :---: |
| [day 1](./tasks/day_1.md) | &#10003; | &#10003; |
| [day 2](./tasks/day_2.md) | &#10003; | &#10003; |
| [day 3](./tasks/day_3.md) | &#10003; | &#10003; |
| [day 4](./tasks/day_4.md) | &#10003; | &#10003; |
| [day 5](./tasks/day_5.md) | &#10003; | &#10003; |
| [day 6](./tasks/day_6.md) | &#10003; | |
| [day 7](./tasks/day_7.md) | &#10003; | &#10003; |
| [day 8](./tasks/day_8.md) | | |
| [day 9](./tasks/day_9.md) | | |
| [day 10](./tasks/day_10.md) | | |
| [day 11](./tasks/day_11.md) | | |
| [day 12](./tasks/day_12.md) | | |
| [day 13](./tasks/day_13.md) | | |
| [day 14](./tasks/day_14.md) | | |
| [day 15](./tasks/day_15.md) | | |
| [day 16](./tasks/day_16.md) | | |
| [day 17](./tasks/day_17.md) | | |
| [day 18](./tasks/day_18.md) | | |
| [day 19](./tasks/day_19.md) | | |
| [day 20](./tasks/day_20.md) | | |
| [day 21](./tasks/day_21.md) | | |
| [day 22](./tasks/day_22.md) | | |
| [day 23](./tasks/day_23.md) | | |
| [day 24](./tasks/day_24.md) | | |
| [day 25](./tasks/day_25.md) | | |


### How to Use

1. Add a New Day:

 - Create `src/dayDD.rs` (you can copy `src/dayXY.rs` as a template).
 - Create `input/DD.txt` -> INPUT
 - Create `input/DD_sample.txt` -> SAMPLE
 - In `src/lib.rs`:
   - Add `pub mod dayDD`;
   - Add a new arm to the `match day` expression in the `solve` function:

```
DD => match part {
    1 => dayDD::part1(input).to_string(),
    2 => dayDD::part2(input).to_string(),
    _ => "Invalid part".to_string(),
},
```

   - Implement tests and solution in `src/dayDD.rs`.

2. Run a Solution:

- The `cargo run` command requires `--` to separate Cargo's arguments from your program's arguments.
   - fe.: run **Day 1, Part 1** -> with input:

```
cargo run -- 1 1
```

   - fe.: run **Day 1, Part 2** with sample:

```
cargo run -- 1 2 --sample
```

3. Run Tests:

- Run all tests: ```cargo test```
- Run tests for a specific day: ```cargo test --package aoc_2024 --lib day01```

#### How to quickly transform the Advent of Code tasks:

1. on the page Select: `View page Source`
2. Save the code between: `<article> ... </article>` into a `day.html`
3. Run: `python TRANSFORM.py`
4. TaDa there will occur output in the console output and file: `day.md`

<author> [**Piotr Parysek**](mailto:piotr.parysek@outlook.com)