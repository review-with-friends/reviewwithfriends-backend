FROM ubuntu:latest

ARG JWT_KEY
ARG TWILIO
ARG DB_CONNECTION

WORKDIR /bout

ENV JWT_KEY=$JWT_KEY
ENV TWILIO=$TWILIO
ENV DB_CONNECTION=$DB_CONNECTION

# Copy our build
COPY ./target/release/bout-backend ./

CMD ["/bout/bout-backend"]

EXPOSE 80