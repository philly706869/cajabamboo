FROM node:22-bookworm

RUN apt-get update && apt-get install -y \
  wget \
  gnupg2 \
  git \
  && apt-get clean

RUN npm i -g npm@latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
  && . "$HOME/.cargo/env" \
  && rustup component add rustfmt

WORKDIR /cajabamboo
