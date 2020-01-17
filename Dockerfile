FROM rust:1.40-stretch

# Dependancies.
RUN apt-get update --fix-missing \
    && apt-get install -y \
        cmake \
        libssl-dev \
        pkg-config \
        zlib1g-dev

# Semantic Release (https://github.com/semantic-rs/semantic-rs).
RUN cargo install semantic-rs

# Copy source.
WORKDIR /usr/src/distance
COPY . .

# Default command runs tests.
CMD ["cargo" , "test"]
