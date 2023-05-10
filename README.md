# eightfish-example-todo

An example project to demo how a simple to do list can be implemented on top of eightfish ODA platform.

# Packages

1. backend, eighitfish based application exposing an API for todo list
2. frontend, yew based frontend application consume the API provided by backend package. implemented based on: https://github.com/yewstack/yew/tree/master/examples/todomvc

# Get Started


## Prerequisites

We assume that you have done the EightFish [basic tests](https://github.com/eightfish-org/eightfish/blob/master/docs/docker.md). And at least you have to keep three images : 

```
REPOSITORY                  TAG               IMAGE ID       CREATED         SIZE
eightfish-m2-http_gate      latest            1c24cd5f171c   2 weeks ago     139MB
eightfish-m2-subxtproxy     latest            37a05c13db96   2 weeks ago     85.6MB
eightfish-m2-subnode        latest            a3d12e0f1eae   2 weeks ago     146MB
  
```


## First step: build the first stage image

In the root directory of this project, run

```
./build_docker.sh
```

This will build a image named `eightfish-todo-build`.

## Second step: build backend and frontend images

Go to the `docker` subdir, to build the todo-backend image and the todo-frontend image.

```
cd docker
./build_app_images.sh
```

This will build other two images: `eightfish-todo_backend` and `eightfish-todo_frontend`.

Now you can get like:

```
REPOSITORY                  TAG               IMAGE ID       CREATED         SIZE
eightfish-todo_frontend     latest            1f390122e8fb   4 hours ago     3.73GB
eightfish-todo_backend      latest            b5377bd3b151   5 hours ago     140MB
eightfish-todo-build        latest            591748a6928e   5 hours ago     4.69GB
eightfish-m2-http_gate      latest            1c24cd5f171c   2 weeks ago     139MB
eightfish-m2-subxtproxy     latest            37a05c13db96   2 weeks ago     85.6MB
eightfish-m2-subnode        latest            a3d12e0f1eae   2 weeks ago     146MB
```

## Third step: start services

In this step, we will boot up all services:

```
cd docker
docker compose -f docker-compose-1node.yml up
```

Among of all, the frontend service would take some time (maybe 60s) to take up, so be patient to wait for its up.

The frontend service would expose port: 8088 to host.

## Visit and test

You can open a web browser, visit: `http://127.0.0.1:8088`, wait a moment, it will show:

![todo](https://github.com/eightfish-org/eightfish_assets/blob/4ce31e2a10944d5e382c5b8f8d4cad5412b43e20/eightfish-todo-example.png)

And you can fill new item into the input box. Caution: At this moment, one post action would take more than 10 seconds (about 2 blocks generation period) to finish, so you'd better to wait for some seconds after each posting.

## Use polkadot.js to look up

Yet you can use https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer to look up the info from the blockchain.

