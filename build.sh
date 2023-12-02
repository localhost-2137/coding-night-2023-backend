#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $SCRIPT_DIR
sudo docker build -t filipton/smarty-backend:latest .

cd $SCRIPT_DIR/nginx
sudo docker build -t filipton/smarty-nginx:latest .

sudo docker push filipton/smarty-backend:latest
sudo docker push filipton/smarty-nginx:latest
