# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory in the Docker image
WORKDIR /usr/src/spinapp

# Install the necessary dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    libssl-dev \
    pkg-config

# Add the wasm32 target
#RUN rustup target add wasm32-unknown-unknown

# Copy the current directory contents into the Docker image
COPY . .

RUN rm spin.toml
RUN mv azure.spin.toml spin.toml

WORKDIR /usr/local/bin

RUN curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

WORKDIR /usr/src/spinapp

RUN rustup target add wasm32-wasi

# Build the application
RUN spin build

# Set the startup command to run your binary
CMD ["spin", "up","--listen","127.0.0.1:80"]

