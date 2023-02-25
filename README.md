# eightfish-example-todo

An example project to demo how a simple to do list can be implemented on top of eightfish ODA platform.

# Packages

1. backend, eighitfish based application exposing an API for todo list
2. frontend, yew based frontend application consume the API provided by backend package. implemented based on: https://github.com/yewstack/yew/tree/master/examples/todomvc

# Get Started

Spin up todo backend module

```
cd backend
spin build
spin up --spin up --follow-all
```

Build and test todo mvc front-end

```
cd frontend
trunk serve --open --port [port_number]
```
