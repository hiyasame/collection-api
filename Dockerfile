FROM rust

WORKDIR /usr/src/collection-api
COPY . .

# ENV DATABASE_URL ******

RUN cargo install --path .

CMD ["collection-api"]