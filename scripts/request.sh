#!/bin/bash
set -u
set -e
curl -sS https://raytracer-svmwbzcaaq-uw.a.run.app/render\?time=$1 --output output/remote/$1.tar
mkdir -p output/remote/$1
tar -xf output/remote/$1.tar -C output/remote/$1
rm output/remote/$1.tar