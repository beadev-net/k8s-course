# Build image
FROM rust:1.69-alpine3.17 as builder
WORKDIR /usr/src/http_mock_server
COPY . .
RUN echo $(nproc)
RUN cargo install -j=$(nproc) --path .

# Production image
FROM alpine:3.17
RUN apk update
WORKDIR /usr/local/bin/
COPY --from=builder     /usr/src/http_mock_server/.env.example      .env
COPY --from=builder     /usr/src/http_mock_server/mock_data.json    mock_data.json
COPY --from=builder     /usr/src/http_mock_server/src/sample.mp4        sample.mp4
COPY --from=builder     /usr/local/cargo/bin/http_mock_server       http_mock_server

EXPOSE 7878

CMD ["http_mock_server", "-f", "./mock_data.json"]