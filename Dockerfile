# syntax=docker/dockerfile:latest
FROM rust:1.52-buster AS build-env
ARG TAG=v0.15.0
WORKDIR /root
RUN git clone -b $TAG --depth 1 https://github.com/informalsystems/ibc-rs
WORKDIR /root/ibc-rs
RUN --mount=type=cache,target=./target/ \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --release; \
  cp /root/ibc-rs/target/release/hermes /usr/bin/hermes

FROM ubuntu
SHELL [ "/bin/bash", "-cx" ]
LABEL maintainer="pratikbin"
ARG TINI_VERSION=v0.19.0
ARG USER=assetmantle
ARG USERID=1001
ARG GROUP=assetmantle
ARG GROUPID=1001
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /bin/tini
RUN echo "root:${ROOT_PASS}" | chpasswd; \
  groupadd -g ${GROUPID} ${GROUP}; \
  useradd -m -d /home/${USER} -s /bin/bash -g ${GROUPID} -G ${GROUP} -u ${USERID} ${USER}; \
  chmod +x /bin/tini
USER ${USER}:${GROUP}
WORKDIR /home/${USER}/.hermes
WORKDIR /home/${USER}
COPY --chown=${USER}:${GROUP} --from=build-env /usr/lib/x86_64-linux-gnu/libssl.so.1.1 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
COPY --chown=${USER}:${GROUP} --from=build-env /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1
COPY --chown=${USER}:${GROUP} --from=build-env /usr/bin/hermes /usr/bin/hermes
ENTRYPOINT ["/usr/bin/hermes"]
