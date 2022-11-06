# Setup

Set `TWITTER_BEARER` in the `.env` file

1. `docker-compose build`
1. `docker-compose up`
1. `docker-compose run --rm image_scout diesel migration run`
1. `docker-compose run --rm image_scout just reseed`
