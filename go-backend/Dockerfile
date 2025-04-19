FROM golang:1.23.5-bookworm AS builder
WORKDIR /usr/src/app
COPY go.mod go.sum ./
RUN go mod download && go mod verify
# RUN go test -v

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -v -o /usr/src/app/go-recipe-gallery ./...

# FROM alpine:3.16.3
# WORKDIR /usr/src/app
# COPY --from=builder /usr/src/app/go-recipe-gallery /usr/src/app/go-recipe-gallery
# EXPOSE 3333
# CMD ["/usr/src/app/go-recipe-gallery"]

FROM gcr.io/distroless/static-debian12:debug-nonroot
COPY --from=builder /usr/src/app/go-recipe-gallery /
EXPOSE 3333
ENTRYPOINT []
CMD ["/go-recipe-gallery"]
