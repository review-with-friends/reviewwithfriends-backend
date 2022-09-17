FROM ubuntu:20.04

ARG MONGO_DB

WORKDIR /mob

# Copy our build
COPY ./target/debug/mob-backend ./
COPY ./Rocket.toml ./

ENV ROCKET_DATABASES=$MONGO_DB

CMD ["/mob/mob-backend"]

EXPOSE 80