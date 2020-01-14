FROM rust:1.40-stretch

WORKDIR /usr/src/distance
COPY . .

CMD ["cargo" , "test"]
