#!/bin/bash

docker build -t webmap:latest .
docker run -d -p 8000:8000 -p 21:21 webmap:latest

echo "YOUR ACCESS TOKEN: "
docker exec -ti $(docker ps -a -q) /root/token
