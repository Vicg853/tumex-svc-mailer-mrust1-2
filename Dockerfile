##* Building stage
FROM rust:nightly as BUILD

WORKDIR /build
COPY . . 

RUN cargo build --release

##* Run stage
FROM scratch

WORKDIR /run
COPY --from=BUILD /build/target/release/rust-mailer-api /rust-mailer-api

CMD [ "/rust-mailer-api" ]