# A Rusty Game of Life Implementation

This crate is an implementation of John Conway's [Game of
Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) that takes full
advantage of multi-core CPUs with the [rayon](https://crates.io/crates/rayon)
and [piston](https://crates.io/crates/piston) libraries on the back-end.
