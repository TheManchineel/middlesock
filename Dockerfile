
# Use the official Rust image as the base image
FROM rust:alpine3.18 AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the app in release mode
RUN apk add --no-cache alpine-sdk libressl-dev musl-dev pkgconfig && cargo build --release --locked

# Create a new stage with a minimal image to run the app
FROM alpine:3.18

# Set the working directory to /app
WORKDIR /app

# Copy the built artifact from the previous stage to the container
COPY --from=builder /app/target/release/middlesock ./

# Run the app
ENV ROCKET_ENV=production ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8013
CMD ["./middlesock"]
