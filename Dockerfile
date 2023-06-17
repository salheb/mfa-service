# Build: docker build -t token-service . 
#         && docker images
# Run: docker run -p 80:80 token-service // docker run -p <host-port>:<container-port> <image-name>
# Test: curl http://localhost/

# Use rust-based image for container; rustc version 1.43.1
FROM rust:1.67

# Set working directory in container
RUN mkdir /usr/src/token-service
WORKDIR /usr/src/token-service

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .
COPY LICENSE .
COPY .env .env

# Build release application
RUN cargo build --release

# Expose listening port for application
EXPOSE 80

# Run the application
CMD ["target/release/token-service"]