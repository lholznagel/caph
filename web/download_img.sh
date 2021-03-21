#!/bin/bash

set -e

# Downloads missing images and places them into the assets folder

FOLDER="/home/pleb/dev/caph/web/src/assets/img_64"
ITEMS=$(curl -q "http://192.168.178.199:10101/api/market/items" | jq -r '.[]')

for x in $ITEMS
do
  if ! [ -f "${FOLDER}/${x}_64.png" ]; then
    cp ${FOLDER}/empty_64.png ${FOLDER}/${x}_64.png
    wget -q -O ${FOLDER}/${x}_64.png https://image.eveonline.com/Type/${x}_64.png
    echo "${x} downloaded"
  fi
done
