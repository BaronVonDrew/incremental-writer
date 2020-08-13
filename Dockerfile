FROM rust:1.45

WORKDIR /usr/src/myapp
COPY . .

CMD ["cargo", "test", "--", "--nocapture"]