#!/bin/bash
echo "build image"
docker build -t cms-backend .
echo "start container"
if [[ -n ${DATABASE_URL+x} ]]; then 
    echo "Env is set"
else
    echo "Env is not set"
    export DATABASE_URL="mysql://root:pass@localhost:3306/ferienwvk_db1"
fi
docker run --rm -p 8001:8001 -p 3306:3306 --env DATABASE_URL="${DATABASE_URL}" cms-backend