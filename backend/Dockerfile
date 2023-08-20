ARG VERSION=1
FROM rust:${VERSION}-buster AS builder

############################
# STAGE 1 build the binary
############################

# The below line should match the package name in Cargo.toml
ARG APP_NAME="veloxide-server"

RUN cargo new --bin ${APP_NAME}

WORKDIR /${APP_NAME}

COPY ./crates/${APP_NAME} ./
COPY ./contracts/ ./
COPY ./contracts/ ./${APP_NAME}

ENV CONTRACTS_PATH="./contracts"
ENV SQLX_OFFLINE=true
ARG DATABASE_KIND="postgres"
ARG FEATURES="tracing,graphql,frontend,openapi,bunyan"

RUN apt-get update && apt-get install -y protobuf-compiler

RUN cargo build --release --features ${DATABASE_KIND},${FEATURES} --no-default-features

############################
# STAGE 2 build a small image
############################

FROM debian:buster-slim

ARG APP=/usr/src/app
ARG GOOGLE_CLIENT_ID
ARG GOOGLE_CLIENT_SECRET

RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Europe/Amsterdam \
    APP_USER=appuser \
    AUTHZ_ENABLED=false \
    OBSERVABILITY_SERVICE_NAME="veloxide-server" \
    OTEL_EXPORTER_OTLP_ENDPOINT="http://tempo:4317" \
    RUST_LOG="info" \
    DATABASE_URL="postgresql://postgres:thisisnotsecure@postgres:5432/veloxidedb" \
    HTTP_PORT="8080" \
    TOKEN_KEY="ca3e5d12-61dd-4d1e-9d98-10c308d31531" \
    TOKEN_DURATION_MINUTES="1440" \
    POLICY_SERVER_URL="http://opa:8181/v1/data/httpapi/authz" \
    GOOGLE_REDIRECT_URL="http://localhost:8080/auth/google/callback"

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /veloxide-server/target/release/veloxide-server ${APP}/veloxide-server

RUN chown -R $APP_USER:$APP_USER ${APP}

EXPOSE 8080

USER $APP_USER
WORKDIR ${APP}

# HEALTHCHECK --interval=5s --timeout=3s --start-period=5s --retries=3 CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["./veloxide-server"]
