FROM golang:alpine

RUN go install github.com/cosmtrek/air@latest

WORKDIR /opt/code

# The directory will be mounted
# COPY . .

CMD [ "air", "-c", ".air.toml" ]
