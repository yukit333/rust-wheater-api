FROM rust:1.38 AS build
WORKDIR /usr/src

# Enable Nigthly Build
RUN rustup default nightly

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-gnu --toolchain=nightly

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new hello_cargo
WORKDIR /usr/src/hello_cargo
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-gnu --path .

# Copy the statically-linked binary into a scratch container.
FROM debian:stretch-slim
COPY --from=build /usr/local/cargo/bin/hello_cargo .
CMD ["./hello_cargo"]
