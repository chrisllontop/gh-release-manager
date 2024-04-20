# Use an official Rust image
FROM rust:slim

# Copy the source code into the container
WORKDIR /usr/src/myapp
COPY . .

# Compile the Rust application
RUN cargo build --release

# Run the binary
ENTRYPOINT ["./target/release/gh_release_manager"]
