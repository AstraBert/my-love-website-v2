# Build stage
FROM rust:1 AS builder

# Install dioxus-cli and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

WORKDIR /app

# Copy your project files
COPY . .

# Build the web bundle
RUN dx build --release --platform web

# Production stage - serve with nginx
FROM nginx:alpine

# Copy the built files from the correct location
COPY --from=builder /app/target/dx/love-sentences/release/web/public /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]