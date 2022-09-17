FROM debian:buster-slim

ARG MONGO_DB

WORKDIR /mob

# Copy our build
COPY ./target/release/mob-backend ./
COPY ./Rocket.toml ./

ENV ROCKET_DATABASES=$MONGO_DB

CMD ["/mob/mob-backend"]

EXPOSE 80