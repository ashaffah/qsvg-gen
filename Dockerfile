FROM rust:1.76.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .

RUN rustup default 1.76.0
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]