FROM rust

RUN cargo install cargo-watch && \
    mkdir -p /opt/code && \
    touch /opt/code/dev-entry.sh && \
    chmod +x /opt/code/dev-entry.sh

CMD [ "/opt/code/dev-entry.sh" ]
