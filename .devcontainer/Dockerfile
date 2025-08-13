FROM rust:trixie

RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
  && apt-get update && apt-get install -y \
  nodejs \
  wget \
  gnupg2 \
  git \
  && apt-get clean \
  && npm i -g npm@latest \
  && rustup component add rustfmt

WORKDIR /cajabamboo
