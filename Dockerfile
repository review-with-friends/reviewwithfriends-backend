FROM ubuntu:latest

WORKDIR /mob

# Copy our build
COPY ./target/release/mob-backend ./
COPY ./Rocket.toml ./

CMD ["/mob/mob-backend"]

EXPOSE 80