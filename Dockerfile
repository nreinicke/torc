# Usage:
#   # Build
#   docker build --build-arg VERSION=0.14.0 -t ghcr.io/daniel-thom/torc:0.14.0 .
#
#   # Push to GitHub Container Registry
#   echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
#   docker push ghcr.io/daniel-thom/torc:0.14.0
#
#   # Run with required env vars
#   docker run -d -p 8080:8080 \
#     -e TORC_AUTH_FILE=/data/htpasswd \
#     -e TORC_ADMIN_USERS=admin \
#     -v ./htpasswd:/data/htpasswd:ro \
#     -v torc-data:/data \
#     ghcr.io/daniel-thom/torc:0.14.0
#
#   # Run torc CLI
#   docker run --rm ghcr.io/daniel-thom/torc:0.14.0 torc --version

FROM alpine:3.23

ARG VERSION
RUN test -n "$VERSION" || (echo "ERROR: VERSION build arg is required" && exit 1)

LABEL org.opencontainers.image.title="torc" \
      org.opencontainers.image.description="Distributed workflow orchestration system" \
      org.opencontainers.image.url="https://github.com/NatLabRockies/torc" \
      org.opencontainers.image.source="https://github.com/NatLabRockies/torc" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.licenses="BSD-3-Clause"

# Download release binaries and remove curl afterward to keep image small
# This is going to use pixi as soon as images are stored on conda-forge.
RUN apk add --no-cache ca-certificates curl sqlite tmux && \
    curl -fsSL "https://github.com/NatLabRockies/torc/releases/download/v${VERSION}/torc-x86_64-unknown-linux-musl.tar.gz" \
      | tar xz -C /usr/local/bin/ && \
    apk del curl

# Create data directory with OpenShift-compatible permissions.
# OpenShift runs containers as arbitrary non-root UIDs but always in group 0 (root).
RUN mkdir -p /data && chgrp -R 0 /data && chmod -R g=u /data

VOLUME /data

COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

EXPOSE 8080

USER 1001

ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["torc-server", "run"]
