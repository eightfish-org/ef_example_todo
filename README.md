# eightfish-example-todo

An example project to demo how a simple to do list can be implemented on top of eightfish ODA platform.

# Packages

1. backend, eighitfish based application exposing an API for todo list
2. frontend, yew based frontend application consume the API provided by backend package. implemented based on: https://github.com/yewstack/yew/tree/master/examples/todomvc

# Get Started


## Prerequisite

We assume that you have done the EightFish [basic tests](https://github.com/eightfish-org/eightfish/blob/master/docs/docker.md). And at least you have to keep three images : 

```

  
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

Now you can get:

```

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

![]()

And you can fill in new item in the input box. Caution: At this moment, one post action would take more than 10 seconds (about 2 blocks generation period) to finish.



