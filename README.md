# Rust Conway's Game of Life

## Overview

This is an implementation of Conway's Game of Life in Rust. This was a chance
for me to practice learning to program in Rust some more, and learn some
features of Rust that I was not already familiar with.

Big picture concepts that I wanted to work on in this are:
*  The module system
*  Testing
*  Documenting functions and structs

## Rules

From https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life:

```
Every cell interacts with its eight neighbours, which are the cells that are
horizontally, vertically, or diagonally adjacent. At each step in time, the
following transitions occur:
*  Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
*  Any live cell with two or three live neighbours lives on to the next generation.
*  Any live cell with more than three live neighbours dies, as if by overpopulation.
*  Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
```

## Difficulties Encountered & Lessons Learned

### Closures and Higher-Order Functions

For statements involving a chain of HOF's, it can be helpful to use closures
rather than declaring the logic inline. This means that fewer methods are
included in our `impl` blocks, as well as making our pipelines more readable.

### Ownership, Borrowing

This project required gaining a better understanding of how Rust's ownership
and borrowing systems work. While it can be difficult initially to work with,
it also prevents subtle memory bugs from being introduced to the project.

### Testing

One thing that is especially helpful is Rust's testing system. You can unit
test the functions in a module, including private functions, by placing a
test module at the bottom of a function with the `#[cfg(test)]`, attribute.

Integration tests found in the `tests/` directory of the crate will only be
able to access publicly exposed functions and structs, so these provide two
different ways to check that code is functioning correctly.

### Documenting Strings

Documentation comments can be placed above a function with comments marked
with `///`. A neat fact about these comments is that they do support Markdown!
