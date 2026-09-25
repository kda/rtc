# rtc
Rust Text Calculator

Text-based calculator written in Rust.

[![Rust](https://github.com/kda/rtc/actions/workflows/rust.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust.yml)
[![Windows](https://github.com/kda/rtc/actions/workflows/rust_on_windows.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust_on_windows.yml)
[![MacOS](https://github.com/kda/rtc/actions/workflows/rust_on_macos.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust_on_macos.yml)

# todo
## features
-   calc
    -   absolute
    -   trunc
    -   frac
- 	add color (if terminal capable and/or if flag)
- 	check terminal size at startup for available space (exit gracefully if insufficient.)
-   display commas (modal, also consider EURO style (.  <-> ,)) (possibly detect based on locale?)
-   develop tests to cover all features
-   detect and enforce maximum length of entry of value
-   consider a help screen which labels the accumulator and pending operation and current value (quick tour?)
-   add marker in display to show mode
-   possibly show first line of display: summary of last action executed
-   have history of operations, and rewind and fast-forward
-   add 'x' for multiply
## refactor
-   reconsider BigText using width and height
    - also, could be optional, based on command line
## bug
-   conversion breakages:
    -   Octal (Decimal) (27) -> ??? (Scientific) (crashes)
-   entered value does not preserve digits: examples:
    -   load constant E when in integer mode, then switch to decimal (no digits right of decimal point)
    -   load constant E when in decimal mode with 2 fixed, then switch to fixed 9 (all zeroes in digits 3-9)
    -   possible solution: store ValuePair with String accumulator
-   first tilde after equals does not change display (not accumulator)
## command line
-   confirm quit request
## preferences (also, all available via command line)
-   start in decimal or scientific mode (or integer (default))
-   start in hex, oct, or bin base (or decimal (default))
-   retain values of storage registers between runs
-   support upper case hex display
-   support upper case e in scientific display (maybe same as hex UPPER)
-   clear screen (before, after)
-   number of significant digits
-   color mode
-   display commas (modal, also consider EURO style (.  <-> ,))
-   display memory registers
-   confirm quit request

## discarded attempts
-   BigText Result: cargo add ratatui tui-big-text
    - Octant size did not render correctly
    - Sextant did not either
    - Quadrant was too big


# Developer hints
-   Insta Review
    -   `cargo install cargo-insta`
    -   `cargo insta review`
