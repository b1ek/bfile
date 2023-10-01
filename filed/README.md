# fileD - file daemon
This is a part of blek! File that is responsible for serving and uploading files.  
This module is released under the GPLv3 with additions, copy of which is included in the top level of this repository.

## Building
To get started with this, copy either `Dockerfile.dev` or `Dockerfile.prod` to `Dockerfile`, depending on your environment.

Then either build it manually or start it up using the `docker-compose.yml` file, which is provided in the top level directory.

## Deploying notes
Files will be saved in `/opt/useruploads`. Mount that directory into a volume or host directory to easily back up the data.
