##* Building stage
FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim as BUILD

WORKDIR /build
COPY .dockerignore .dockerignore
COPY . . 

RUN cargo build --release

##* Run stage
FROM scratch

WORKDIR /run
COPY --from=BUILD /build/target/release/rust-mailer-api /rust-mailer-api
EXPOSE 8000

CMD [ "/rust-mailer-api" ]