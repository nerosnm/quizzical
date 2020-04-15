FROM rust:1.42.0 AS build
WORKDIR /src

# Install nightly rust
RUN rustup update nightly && rustup default nightly

# Copy sources to /src
COPY . .
# Build app
RUN cargo build --release

# Copy compiled app to /app
RUN mkdir -p /app
RUN cp target/release/zical /app/

FROM ubuntu:18.04 AS run
WORKDIR /app

# Copy compiled app from build phase
COPY --from=build /app/zical .

# Define the entrypoint for running
CMD ROCKET_PORT=$PORT /app/zical
