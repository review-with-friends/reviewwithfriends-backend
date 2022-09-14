FROM debian:buster-slim

WORKDIR /mob

# Copy our build
COPY ./target/release/mob-backend ./
COPY ./Rocket.toml ./

#COPY ./mob.spacedoglabs.com.cer /
#COPY ./mob.spacedoglabs.com.pem /

CMD ["/mob/mob-backend"]

EXPOSE 80