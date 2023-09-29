# --- build ---
FROM rust:alpine as builder

WORKDIR /opt/build
COPY . .

RUN cargo b -r

# --- deploy ---
FROM alpine

COPY --from=builder /opt/build/target/release/filed /bin/filed
CMD [ "/bin/filed" ]