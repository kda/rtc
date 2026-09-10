# rtc
Rust Text Calculator

Text-based calculator written in Rust.

[![Rust](https://github.com/kda/rtc/actions/workflows/rust.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust.yml)
[![Windows](https://github.com/kda/rtc/actions/workflows/rust_on_windows.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust_on_windows.yml)
[![MacOS](https://github.com/kda/rtc/actions/workflows/rust_on_macos.yml/badge.svg)](https://github.com/kda/rtc/actions/workflows/rust_on_macos.yml)

# todo
## refactor
-   re-write keys, help, names, consider using Fixed Arrays (to avoid all copying of iters, etc)
-   reconsider BigText using width and height
    - also, could be optional, based on command line
## features
- 	add color (if terminal capable and/or if flag)
- 	check terminal size at startup for available space (exit gracefully if insufficient.)
-   display commas (modal, also consider EURO style (.  <-> ,)) (possibly detect based on locale?)
-   build out help system
-   develop tests to cover all features
-   add 'V' to show version
-   detect and enforce maximum length of entry of value
-   implement memory registers
## bug
-   Esc clears accumulator, but it still shows a value
## command line
-   add clap to support version and other command line flags
-   support -V for version
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
