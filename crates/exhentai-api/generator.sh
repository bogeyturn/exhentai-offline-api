#!/bin/bash
if [ $# -ne 1 ];then
  echo "Error: please provide one argument:"
  echo "failed: Failed db"
  echo "api: API dump"
  echo "gp: GP Crawl"
  echo "rating: ..."
  exit 1
fi

if [[ $1 != "failed" && $1 != "api" && $1 != "gp" && $1 != "rating" ]]; then
  echo "Error: Invalid argument. Please provide failed, api, gp, rating"
  exit 1
fi

if [ -f .env ]; then
  export $(echo $(cat .env | sed 's/#.*//g'| xargs) | envsubst)
fi

if [ "$1" == "api" ]; then
  DATABASE=$DATABASE_URL_API_DUMP
  CONFIG=diesel_api_dump.toml
elif [ "$1" == "gp" ]; then
  DATABASE=$DATABASE_URL_GP_CRAWL
  CONFIG=diesel_gp_crawl.toml
elif [ "$1" == "rating" ]; then
  DATABASE=$DATABASE_URL_RATING
  CONFIG=diesel_rating.toml
else
  DATABASE=$DATABASE_URL_FAILED
  CONFIG=diesel_failed.toml
fi

diesel migration run --config-file "$CONFIG" --database-url "$DATABASE"