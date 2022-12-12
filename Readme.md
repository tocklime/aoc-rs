# Advent of code, all solutions, in Rust.

This is all my solutions to advent of code, and accompanying runner harness I've built up over the years.

Whilst working on a solution,

```bash
$ cargo make watch
```

Will find the most recently edited solution, compile just that (using the aoc_latest macro), and run it in release mode.

```bash
$ cargo make run_all
```

Will compile and run all solutions for all years.

In recent years, I've committed as soon as I get the answer solved, then again after I've tidied up, so if you want to see the mess I create under time pressure, check the ['as solved'](https://github.com/tocklime/aoc-rs/search?q=as+solved&type=commits) commits :)

## 2022 further ideas notes

* day 11 - I want to try solving this where each Monkey runs on a separate thread.