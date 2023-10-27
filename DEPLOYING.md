# Deploying a production instance
Hi fellow sysadmins!
First of all, I want to thank you for using my piece of software.
The instructions can be found below

## Deploying a basic instance
To deploy a basic instance for general public use, follow these simple steps:
1. Clone this repo
2. Copy `docker-compose.prod.yml` to `docker-compose.yml` and edit it to fit your environment
3. Now, there are a few config files that need to be edited by you: `.env`, `filed/.env` and `janitord/.env`. Each directory contains an `.env.example`, and the configuration is pretty straightforward. However, if you are lost check this out: [filed config](#filed-configuration), [janitord config](#janitord-configuration).
4. Configure fileD using `filed/config/filed.toml`. The example is in the same folder. Example contains a lot of self-documenting comments, so it should be pretty simple too.
5. Set `REDIS_PASS` to a secure long string. Not exactly required, but this is something you would want to do
6. Create and start containers with `docker-compose up -d`
7. Route your top level reverse proxy to the `caddy` service or to the port that you opened via the docker compose file.

## FileD configuration
Unless you are running in some kind of super customized docker compose environment, just copying the `.env.example` to `.env` should be enough to get it to run.

Don't forget to set the `REDIS_PASS` to the same value across all services

## JanitorD configuration
Same as [filed config](#filed-configuration), don't forget to set `REDIS_PASS` to a valid value