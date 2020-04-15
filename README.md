# Quizzical

![Rust](https://github.com/nerosnm/quizzical/workflows/Rust/badge.svg)
![Docker](https://github.com/nerosnm/quizzical/workflows/Docker/badge.svg)

Quizzical is a lightweight service (frontend and backend) for running virtual pub quizzes.

## Build

The frontend is in `quiz/` and the backend is in `zical/`.

### Cargo

To build the backend using `cargo`, simply run `cargo build` from the root directory.

### Docker

To build the backend using `docker`, run:

```bash
$ docker build -t zical -f zical/Dockerfile .
```

