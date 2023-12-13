# 🎄 Advent Of Code 2023

![rust logo][rust-badge] ![solutions][solutions-badge]

Hi! These are my Rust solutions for the [Advent of Code 2023][advent-of-code].

<div align="center">

|       Day | Name                            | Source       | Part 1 | Part 2 |   Time 1 |  Time 2 |
| --------: | ------------------------------- | ------------ | :----: | :----: | -------: | ------: |
|  [1][p01] | Trebuchet?!                     | [01.rs][s01] |   ⭐   |   ⭐   |  33.1 µs | 42.1 µs |
|  [2][p02] | Cube Conundrum                  | [02.rs][s02] |   ⭐   |   ⭐   |  26.5 µs | 37.2 µs |
|  [3][p03] | Gear Ratios                     | [03.rs][s03] |   ⭐   |   ⭐   |  39.1 µs | 30.7 µs |
|  [4][p04] | Scratchcards                    | [04.rs][s04] |   ⭐   |   ⭐   |  82.4 µs | 84.0 µs |
|  [5][p05] | If You Give A Seed A Fertilizer | [05.rs][s05] |   ⭐   |   ⭐   |  18.7 µs | 23.5 µs |
|  [6][p06] | Wait For It                     | [06.rs][s06] |   ⭐   |   ⭐   |   174 ns |  180 ns |
|  [7][p07] | Camel Cards                     | [07.rs][s07] |   ⭐   |   ⭐   |   150 µs |  157 µs |
|  [8][p08] | Haunted Wasteland               | [08.rs][s08] |   ⭐   |   ⭐   |   389 µs |  2.5 ms |
|  [9][p09] | Mirage Maintenance              | [09.rs][s09] |   ⭐   |   ⭐   |  99.5 µs | 96.5 µs |
| [10][p10] | Pipe Maze                       | [10.rs][s10] |   ⭐   |   ⭐   | 87.6 µs¹ | 138 µs¹ |
| [11][p11] | Cosmic Expansion                | [11.rs][s11] |   ⭐   |   ⭐   |   3.1 ms |  3.1 ms |
| [12][p12] | Hot Springs                     | [12.rs][s12] |   ⭐   |   ⭐   |   1.7 ms | 21.3 ms |
| [13][p13] | Point of Incidence              | [13.rs][s13] |   ⭐   |   ⭐   |  46.0 µs | 48.5 µs |
|           | ...                             |              |        |        |          |         |

**Key**: ⭐ Completed &nbsp;&nbsp; 🎁 In progress &nbsp;&nbsp; 😔 Gave up

_Benchmarked on Intel i7-11800H @ 2.30 GHz (over many samples)._

</div>

<sub>
<i>¹ I noticed after some refactoring that benchmark times got ~2x slower, with the simpler part taking significantly longer. A ~2x speed increase (relative to initial benchmark) was obtained by only testing one part at a time (with the other commented out for dead-code removal)! This may be an extreme sensitivity to the layout of the linked binary and how this is loaded into the instruction cache? Or bad branch prediction?</i>
</sub>

## Acknowledgments

This repository uses a modified version of [this template][template]. Thanks Felix!

## License

Distributed under the MIT Licence. See [LICENCE](LICENCE) for more information.

[rust-badge]: https://img.shields.io/badge/Rust-d55826?logo=rust&style=for-the-badge
[solutions-badge]: https://img.shields.io/badge/solutions-26/50-brightgreen?logo=star&style=for-the-badge
[advent-of-code]: https://adventofcode.com/
[rust]: https://www.rust-lang.org/
[template]: https://github.com/fspoettel/advent-of-code-rust
[p01]: https://adventofcode.com/2023/day/1
[p02]: https://adventofcode.com/2023/day/2
[p03]: https://adventofcode.com/2023/day/3
[p04]: https://adventofcode.com/2023/day/4
[p05]: https://adventofcode.com/2023/day/5
[p06]: https://adventofcode.com/2023/day/6
[p07]: https://adventofcode.com/2023/day/7
[p08]: https://adventofcode.com/2023/day/8
[p09]: https://adventofcode.com/2023/day/9
[p10]: https://adventofcode.com/2023/day/10
[p11]: https://adventofcode.com/2023/day/11
[p12]: https://adventofcode.com/2023/day/12
[p13]: https://adventofcode.com/2023/day/13
[s01]: src/bin/01.rs
[s02]: src/bin/02.rs
[s03]: src/bin/03.rs
[s04]: src/bin/04.rs
[s05]: src/bin/05.rs
[s06]: src/bin/06.rs
[s07]: src/bin/07.rs
[s08]: src/bin/08.rs
[s09]: src/bin/09.rs
[s10]: src/bin/10.rs
[s11]: src/bin/11.rs
[s12]: src/bin/12.rs
[s13]: src/bin/13.rs
