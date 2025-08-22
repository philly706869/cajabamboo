FROM rust:trixie

RUN rustup component add rust-analyzer rust-src rustfmt clippy \
  && cargo install sqlx-cli

RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
  && apt-get update && apt-get install -y nodejs wget gnupg2 git \
  && apt-get clean \
  && npm i -g npm@latest
