# Advent of Code Solutions

This repository contains my solutions to the Advent of Code problems. The problems can be found [here](https://adventofcode.com/2015/events). All programs emitted from these codes require that you pipe the input towards the program. For example:

```bash
set +e
rustc 01.rs
cat input.txt | ./01
```

### Interactivity

There are certain puzzles that require interactivity from the user. Instead of piping the output into the program, you will instead run the program and paste the problem input.

```bash
set +e
rustc 02.rs
./02
```

Such solutions for puzzles will be marked with a `*` in the progress table. They are usually the Intcode programs in year 2019.

## Progress

| Day |         2015             |         2016         |         2017         |         2018         |          2019         |         2020         |         2021         |          2022        |
|:---:|:------------------------:|:--------------------:|:--------------------:|:--------------------:|:---------------------:|:--------------------:|:--------------------:|:--------------------:|
|  1  | [DONE](./2015/01.rs)     | [DONE](./2016/01.rs) | [DONE](./2017/01.rs) |                      | [DONE](./2019/01.rs)  |                      | [DONE](./2021/01.rs) | [DONE](./2022/01.rs) |
|  2  | [DONE](./2015/02.rs)     |                      |                      |                      | [DONE](./2019/02.rs)  |                      | [DONE](./2021/02.rs) | [DONE](./2022/02.rs) |
|  3  | [DONE](./2015/03.rs)     |                      |                      |                      |                       |                      | [DONE](./2021/03.rs) | [DONE](./2022/03.rs) |
|  4  | [DONE](./2015/04.rs)\*\* |                      |                      |                      |                       |                      | [DONE](./2021/04.rs) | [DONE](./2022/04.rs) |
|  5  | [DONE](./2015/05.rs)     |                      |                      | [DONE](./2018/05.rs) | [DONE](./2019/05.rs)  |                      |                      | [DONE](./2022/05.rs) |
|  6  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/06.rs) |
|  7  | [DONE](./2015/07.rs)     |                      |                      |                      |                       |                      |                      | [DONE](./2022/07.rs) |
|  8  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/08.rs) |
|  9  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/09.rs) |
| 10  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/10.rs) |
| 11  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/11.rs) |
| 12  |                          | [DONE](./2016/12.rs) |                      |                      |                       |                      |                      | [DONE](./2022/12.rs) |
| 13  |                          |                      |                      |                      |                       |                      |                      | [DONE](./2022/13.rs) |
| 14  |                          |                      |                      |                      |                       |                      |                      |                      |
| 15  |                          |                      |                      |                      |                       |                      |                      |                      |
| 16  |                          |                      |                      |                      |                       |                      |                      |                      |
| 17  |                          |                      |                      |                      |                       |                      |                      |                      |
| 18  |                          |                      |                      |                      |                       |                      |                      |                      |
| 19  |                          |                      |                      |                      |                       |                      |                      |                      |
| 20  |                          |                      |                      |                      |                       |                      | [DONE](./2021/20.rs) |                      |
| 21  |                          |                      |                      |                      |                       |                      |                      |                      |
| 22  |                          |                      |                      |                      |                       |                      |                      |                      |
| 23  |                          |                      |                      |                      |                       |                      |                      |                      |
| 24  |                          |                      |                      |                      |                       | [DONE](./2020/24.rs) |                      |                      |
| 25  | [DONE](./2015/25.rs)     |                      |                      |                      |                       | [DONE](./2020/25.rs) |                      |                      |

## Other Notes

- \*\* **2015/04**: Because this requires the `md5` crate, it is recommended instead to create a new crate with `md5` as a dependency and copying `2015/04.rs` as the crate's `src/main.rs`
