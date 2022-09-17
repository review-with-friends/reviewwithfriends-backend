FROM ubuntu:20.04

WORKDIR /mob

# Copy our build
COPY ./target/debug/mob-backend ./
COPY ./Rocket.toml ./

CMD ["/mob/mob-backend"]

EXPOSE 80