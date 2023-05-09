#== First stage: this is the build stage for Substrate. Here we create the binary.
FROM docker.io/paritytech/ci-linux:1.68.2-bullseye as builder

WORKDIR /eftodo
COPY . .

# install rust components
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi

# install third tools
RUN cd /tmp/ && curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash && mv spin /usr/local/bin/
RUN cargo install --locked trunk

# compile app
RUN cd backend && spin build 
RUN cd frontend && trunk build 
