| ⚠️ This is in a rather early stage of development and shouldn't be deployed |
| --------------------------------------------------------------------------- |

Even though this project is mature enough to be deployed in a public instance,
this is highly discouraged.  
However, if you do this, be prepared for [DOS](https://en.wikipedia.org/wiki/Denial-of-service_attack) issues and API changes.

# blek! File
blek! File is a free service that would help you with file sharing.

The principle is very simple: you upload a file, then download it from another device. The file will be deleted after 1 download or 30 minutes.

## Licensing
This software is released under GPL3 license, a copyleft license that protects users' freedom by ensuring that all future copies of this software are open source as well.

## Deploying
Simply copy the `docker-compose.yml.example` to `docker-compose.yml`, and `.env.example` to `.env` and edit them if necessary.

The following could be done with these bash commands:
```bash
$ # Notice that those are just for reference; you may not want to 100% copy them
$ cp docker-compose.yml.example docker-compose.yml
$ cp .env.example .env
$ nvim .env # you need to edit this file
# docker-compose up -d # "#" at the start means that the command must be run as root/sudo
$ # It all should me up and running at this point
```