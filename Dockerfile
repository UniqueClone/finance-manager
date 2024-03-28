# Use an official Rust runtime as a parent image
FROM rust:1.77

# Set the working directory in the container to /usr/src/myapp
WORKDIR /usr/src/finance-manager

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

# Set the startup command to run your binary
CMD ["./target/release/finance-manager"]