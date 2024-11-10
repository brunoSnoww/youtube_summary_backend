
# Stage 1: Build the Rust application
FROM rust:1.72 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy any additional necessary files (e.g., models, scripts)
COPY models ./models
COPY whisper.cpp ./whisper.cpp

# Install dependencies for building whisper.cpp and other tools
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    sox \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Build whisper.cpp if necessary
RUN cd whisper.cpp && \
    make

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    sox \
    ffmpeg \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/your_application_binary /usr/local/bin/your_application_binary

# Copy whisper.cpp binary if necessary
COPY --from=builder /app/whisper.cpp/main /usr/local/bin/whisper

# Copy any models or additional resources
COPY --from=builder /app/models /models

# Set environment variables (will be overridden at runtime)
ENV MODEL_PATH=/models/ggml-base.bin

# Expose the application port (if necessary)
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/your_application_binary"]
