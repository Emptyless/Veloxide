ARG VERSION=1
FROM rust:${VERSION}-buster AS builder

############################
# STAGE 1 build the binary
############################

# The below line should match the package name in Cargo.toml
ARG APP_NAME="veloxide-server"

RUN cargo new --bin ${APP_NAME}

WORKDIR ./${APP_NAME}

COPY . ./

ARG DATABASE_KIND="mysql"

RUN cargo prisma generate && cargo build --release --features ${DATABASE_KIND}

############################
# STAGE 2 build a small image
############################

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Europe/Amsterdam \
    APP_USER=appuser

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