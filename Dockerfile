FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye AS build-env
WORKDIR /App

# Copy everything
COPY . ./
WORKDIR /App/auto_build
RUN cargo build --release

# Build runtime image
FROM alpine:latest
WORKDIR /App
COPY --from=build-env /App/auto_build/target/release/auto_build .
COPY --from=build-env /App/auto_build/templates .

WORKDIR /App
ENTRYPOINT ["./auto_build","/project"]
