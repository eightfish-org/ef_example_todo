#== First stage: this is the build stage for Substrate. Here we create the binary.
FROM eightfish-todo-build as builder

#== Second stage: 
FROM docker.io/library/ubuntu:20.04
LABEL description="EightFish:todo_backend"

WORKDIR /eftodo

RUN mkdir -p /eftodo/target/wasm32-wasi/release/

COPY --from=builder /usr/local/bin/spin /usr/local/bin
COPY --from=builder /eftodo/backend/spin.toml /eftodo/spin_todo.toml
COPY --from=builder /eftodo/backend/target/wasm32-wasi/release/todo.wasm /eftodo/target/wasm32-wasi/release/

