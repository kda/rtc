# rtc
Rust Text Calculator

Text-based calculator written in Rust.


# todo
## refactor
-   re-write keys, help, names, consider using Fixed Arrays (to avoid all copying of iters, etc)
-   reconsider BigText using width and height
    - also, could be optional, based on command line
## features
- 	add color (if terminal capable and/or if flag)
- 	check terminal size at startup for available space (exit gracefully if insufficient.)
-   display commas (modal, also consider EURO style (.  <-> ,)) (possibly detect based on locale?)
-   number of significant digits
-   build out help system
-   develop tests to cover all features
-   add modal operation
-   add 'V' to show version
-   detect and enforce maximum length of entry of value
## command line
-   add clap to support version and other command line flags
-   support -V for version
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

## discarded attempts
-   BigText Result: cargo add ratatui tui-big-text
    - Octant size did not render correctly
    - Sextant did not either
    - Quadrant was too big
