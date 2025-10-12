# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

ARG NODE_VERSION=22
ARG APP_NAME=

################################################################################
# Create a stage for building the application.

FROM node:${NODE_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache git

COPY ./client-side /app/client-side
COPY ./package.json /app/package.json
COPY ./turbo.json /app/turbo.json
COPY ./package-lock.json /app/package-lock.json

# Build the application.
RUN npm ci --include dev && \
    npx turbo build --filter $APP_NAME

FROM nginx:stable-alpine-slim AS final
ARG APP_NAME

COPY --from=build /app/client-side/$APP_NAME/dist /usr/share/nginx/html