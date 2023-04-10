FROM rust:buster AS build

# Install necessary system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the application files into the container
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN diesel setup --database-url=postgres://myuser:mypassword@db:5432/mydb

# Build the application with Cargo
RUN cargo build --release

# Create a new container from the Alpine Linux image
FROM alpine:3.14

# Install necessary system dependencies
RUN apk add --no-cache openssl postgresql-libs

WORKDIR /app

# Copy the binary from the build container to the new container
COPY --from=build /app/target/release/backend .

# Expose port 8000 for the Actix web server
EXPOSE 8000

# Set the command to start the Actix web server
CMD ["./backend"]
