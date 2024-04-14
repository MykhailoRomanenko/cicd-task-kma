ARG APP_NAME=cicd-task-kma

FROM rust:1.77.2 AS builder
ARG APP_NAME

RUN cargo new ${APP_NAME}

WORKDIR /${APP_NAME}

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM debian:12.5-slim AS runtime
ARG APP_NAME

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} .

ENV EXECUTABLE_NAME ${APP_NAME}

CMD "./${EXECUTABLE_NAME}"
