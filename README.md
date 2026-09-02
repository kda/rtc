# rtc
Rust Text Calculator

Text-based calculator written in Rust.


# todo
## refactor
-   re-write keys, help, names, consider using Fixed Arrays (to avoid all copying of iters, etc)
## features
-   BigText Result: cargo add ratatui tui-big-text
-   retain values of storage registers between runs
-   add command line parameters
- 	add color (if terminal capable and/or if flag)
- 	check terminal size at startup for available space (exit gracefully if insufficient.)
-   modal: whole numbers or decimal or Scientific
-   display commas (modal, also consider EURO style (.  <-> ,))
-   number of significant digits
## command line arguments
-   optionally retain values of storage registers between runs
-   clear screen (before, after)
