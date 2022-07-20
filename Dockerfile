ARG BASE_IMAGE=rust:1.62.1-slim-buster

FROM $BASE_IMAGE as planner
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.38
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM $BASE_IMAGE as cacher
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.38
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM hub.pri.ibanyu.com/devops/cc-debian10
COPY --from=builder /app/target/release/echo /
CMD ["./echo"]