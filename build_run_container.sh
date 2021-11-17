#!/usr/bin/env bash
docker-compose -f docker/docker-compose.yml build --parallel --compress --no-cache
docker-compose -f docker/docker-compose.yml up --no-build -d
