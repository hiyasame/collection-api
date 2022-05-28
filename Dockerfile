FROM rust

WORKDIR /usr/src/collection-api
COPY . .

RUN touch /usr/local/cargo/config.toml

# cargo 上海交大镜像源
RUN sed -e a\[source.crates-io] /usr/local/cargo/config.toml;\
    sed -e a\registry=\"https://github.com/rust-lang/crates.io-index\" /usr/local/cargo/config.toml; \
    sed -e a\replace-with=\'sjtu\' /usr/local/cargo/config.toml; \
    sed -e a\[source.sjtu] /usr/local/cargo/config.toml; \
    sed -e a\registry=\"https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/\"

# Rustup 清华镜像源
ENV RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup

ENV DATABASE_URL *

RUN cargo install --path .

CMD ["collection-api"]