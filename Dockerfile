FROM ubuntu:latest

WORKDIR /mob

# Copy our build
COPY ./target/release/mob-backend ./

CMD ["/mob/mob-backend"]

EXPOSE 80