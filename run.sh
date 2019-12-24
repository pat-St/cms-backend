#!/bin/sh
rm -r target
echo "build image"
docker build -t cms-backend .
echo "start container"
docker run -d -p 8000:8000 -p 3306:3306 cms-backend
