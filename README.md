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

To do ...

## Difficulties Encountered & Lessons Learned

### Ownership, Borrowing

To do ...

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
