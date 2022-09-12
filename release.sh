#!/bin/sh

# build image locally
docker build . -t registry.digitalocean.com/mob/backend:latest

# push image to registry
docker push registry.digitalocean.com/mob/backend:latest 