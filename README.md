# Zical

![Rust](https://github.com/nerosnm/quizzical/workflows/Rust/badge.svg)

Zical is the backend half of Quizzical, a web app for running virtual pub quizzes. You can find the 
frontend [here](https://github.com/nerosnm/quiz).

## Cargo

To build the backend using `cargo`, simply run `cargo build` from the root directory.

## Docker

To build the backend using `docker`, run:

```bash
$ docker build -t zical -f Dockerfile .
```

