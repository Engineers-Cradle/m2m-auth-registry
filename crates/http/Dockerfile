FROM rust:1.80.1-slim-bullseye

# View app name in Cargo.toml
ARG APP_NAME=m2m-service-auth-registry-http

WORKDIR /app

COPY . .
RUN cargo build --release
RUN cp ./target/release/$APP_NAME /m2m-service-auth-registry-http

EXPOSE 8000

CMD ["/m2m-service-auth-registry-http"]