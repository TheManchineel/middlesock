# Middlesock

**Middlesock** is a simple, lightweight and secure middleware to monitor an [Authentik](https://goauthentik.io/) instance without exposing your admin credentials to native API clients. Written in Rust using the [Rocket](https://rocket.rs/) library.

Intended for use with [Homepage](https://gethomepage.dev/): just run Authentik, then use the Middlesock URL as your Authentik instance URL within `services.yml` without specifying any credentials.

## Installation (Docker)

The extremely lightweight Alpine-based Docker image is available on the GitHub Container Registry:

```bash
docker run -d \ 
    -p 8013:8013 \ 
    -e AUTHENTIK_BASE_URL="https://authentik.company" \ 
    -e AUTHENTIK_API_KEY="MY_API_KEY" \ 
    --name "Middlesock" \ 
    --restart unless-stopped \ 
    ghcr.io/themanchineel/middlesock:latest
```

Or using Docker Compose:

```yml
version: "3.8"
services:
  middlesock:
    image: ghcr.io/themanchineel/middlesock:latest
    expose:
      - 8013
    environment:
      - AUTHENTIK_BASE_URL="https://authentik.company"
      - AUTHENTIK_API_KEY="MY_API_KEY"
    restart: unless-stopped
  homepage:
    image: ghcr.io/gethomepage/homepage:latest
    ... # See Homepage docs
```