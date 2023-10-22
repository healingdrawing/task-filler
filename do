#!/bin/bash

# try to remove the container if it exists. If it doesn't, the process will be continued
docker rm -f carrot

# try to remove the image if it exists. If it doesn't, the process will be continued
docker rmi -f filler

# build the image from the Dockerfile
docker build -t filler .