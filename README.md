# Quizzical

Quizzical is a lightweight service (frontend and backend) for running a virtual pub quiz.

## Build

The frontend is in `quiz/` and the backend is in `zical/`.

### Cargo

To build the backend using `cargo`, simply run `cargo build` from the root directory.

### Docker

To build the backend using `docker`, run:

```bash
$ docker build -t zical -f zical/Dockerfile .
```

