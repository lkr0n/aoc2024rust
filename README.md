# aoc2024

This repo not only includes my solutions to advent of code 2024 problems in
rust but also the test harness used to run them.

Problem solutions must be implemented in a `src/dayXX.rs` file in public
`part1`, `part2` functions for each day. Here the `XX` stands for the day number
taking integer values in `[1, 25]`.

The problems expect their input in an `./input` directory in the
project-root-directory. Similar to the problem solutions, the input files must
be named `input/dayXX.txt`.

If you put your advent of code session cookie inside the env var
`AOC_SESSION_COOKIE`, the harness will download problem input automatically for
you.
