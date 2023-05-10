#== First stage: this is the build stage for Substrate. Here we create the binary.
FROM eightfish-todo-build as builder

#== Second stage: 
#FROM docker.io/library/ubuntu:20.04
FROM docker.io/paritytech/ci-linux:1.68.2-bullseye
LABEL description="EightFish:todo_frontend"

WORKDIR /eftodo

COPY --from=builder /usr/local/cargo/bin/trunk /usr/local/bin
COPY --from=builder /eftodo/frontend/ /eftodo/
