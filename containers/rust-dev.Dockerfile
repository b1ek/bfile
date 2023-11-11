FROM rust

RUN cargo install cargo-watch && \
    mkdir -p /opt/code && \
    touch /opt/code/dev-entry.sh && \
    chmod +x /opt/code/dev-entry.sh

RUN apt update && \
    apt install nodejs npm -y --no-install-recommends && \
    npm i -g uglify-js html-minifier

CMD [ "/opt/code/dev-entry.sh" ]
