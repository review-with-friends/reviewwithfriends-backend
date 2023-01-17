FROM ubuntu:latest

ARG JWT_KEY
ARG TWILIO
ARG DB_CONNECTION
ARG SPACES_KEY
ARG SPACES_SECRET
ARG NR_KEY

WORKDIR /spotster

ENV JWT_KEY=$JWT_KEY
ENV TWILIO=$TWILIO
ENV DB_CONNECTION=$DB_CONNECTION
ENV SPACES_KEY=$SPACES_KEY
ENV SPACES_SECRET=$SPACES_SECRET
ENV NR_KEY=$NR_KEY

RUN apt-get update
RUN apt-get install ca-certificates -y
RUN update-ca-certificates

# Copy our build
COPY ./target/release/spotster-backend ./

CMD ["/spotster/spotster-backend"]

EXPOSE 80