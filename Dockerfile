FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye AS build-env
WORKDIR /App

# Copy everything
COPY . ./
WORKDIR /App/auto_build
RUN cargo build --release

# Build runtime image
FROM debian:buster-slim
COPY ./auto_build/templates /App/templates/
COPY --from=build-env /App/auto_build/target/release/auto_build /App

WORKDIR /App
ENTRYPOINT ["/App/auto_build","/project"]
