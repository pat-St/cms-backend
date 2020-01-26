#!/bin/sh
rm -r target
echo "build image"
docker build -t cms-backend .
echo "start container"
docker run -d -p 8001:8000 -p 3306:3306 --env DATABASE_URL="mysql://root:pass@localhost:3306/ferienwvk_db1" cms-backend
