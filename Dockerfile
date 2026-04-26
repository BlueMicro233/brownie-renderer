FROM public.ecr.aws/docker/library/ubuntu:22.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    g++ cmake make \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN make release
