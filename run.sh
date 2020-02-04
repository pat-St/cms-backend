#!/bin/bash
rm -r target
echo "build image"
docker build -t cms-backend .
echo "start container"
if [[ -n ${DATABASE_URL+x} ]]; then 
    echo "Env is set"
    docker run -d -p 8001:8000 -p 3306:3306 --env DATABASE_URL="${DATABASE_URL}" cms-backend
else
    echo "Env is not set"
    docker run -d -p 8001:8000 -p 3306:3306 --env DATABASE_URL="mysql://root:pass@localhost:3306/ferienwvk_db1" cms-backend
fi