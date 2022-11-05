FROM ubuntu:latest
ARG JWT_KEY
ARG TWILIO
ARG DB_CONNECTION

WORKDIR /mob

ARG JWT_KEY={$JWT_KEY}
ARG TWILIO={$TWILIO}
ARG DB_CONNECTION={$DB_CONNECTION}

# Copy our build
COPY ./target/release/mob-backend ./

CMD ["/mob/mob-backend"]

EXPOSE 80