# Piston Laboratory

A playground for testing [Piston Engine][1].

## Current Binaries

### Creating a window

An example of creating a piston window.

Source: `src\bin\create_window.rs`
To execute: `cargo run --bin create_window`

### Drawing text

An example of drawing text on a piston window.

Source: `src\bin\draw_text.rs`
To execute: `cargo run --bin draw_text`

### Drawing zh-tw text

An example of drawing text in traditional chinese (Taiwan) on a piston window. It is an important experiment to test if Piston can correctly render UTF-8 strings.

Source: `src\bin\draw_zhtw_text.rs`
To execute: `cargo run --bin draw_zhtw_text`

### Conrod GUI

To test if I can draw something using Piston while rendering GUI using [Conrod][2] at the same time. **However, due to lack of tutorials, I cannot make it work properly for now.**

Source: `src\bin\conrod_gui.rs`
To execute: `cargo run --bin conrod_gui`

Reference:
- [The `conrod_core` document][3]
- [The `conrod_piston` document][4]
- [An example to build a Conrod GUI on top of Piston][5]

[1]: https://github.com/PistonDevelopers/piston
[2]: https://github.com/PistonDevelopers/conrod
[3]: https://docs.rs/conrod_core/0.62.0/conrod_core/
[4]: https://docs.rs/conrod_piston/0.62.0/conrod_piston/
[5]: https://github.com/PistonDevelopers/conrod/blob/master/backends/conrod_piston/examples/all_piston_window.rs
