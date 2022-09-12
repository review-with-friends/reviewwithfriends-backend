FROM rust:latest AS builder

RUN rustup install stable-x86_64-unknown-linux-musl

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

# Create appuser
ENV USER=mob
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /mob

COPY ./ .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=gcc-x86-64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=gcc-x86-64-linux-gnu-gcc

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /mob

# Copy our build
COPY --from=builder /mob/target/x86_64-unknown-linux-musl/release/mob-backend ./

# Use an unprivileged user.
USER mob:mob

ENV ROCKET_ADDRESS="0.0.0.0"

CMD ["/mob/mob-backend"]

EXPOSE 8000