# eightfish-example-todo

An example project to demo how a simple to do list can be implemented on top of eightfish ODA platform.

# Packages

1. backend, eighitfish based application exposing an API for todo list
2. frontend, yew based frontend application consume the API provided by backend package. implemented based on: https://github.com/yewstack/yew/tree/master/examples/todomvc

# Get Started

To make this todo work, we need to prepare database first.

1. we need a database named: `spin_dev`, reference: [init_pg.sh](https://github.com/eightfish-org/eightfish/blob/master/init_pg.sh)
2. we need the following schema for todo application. reference: [create.sql](https://github.com/eightfish-org/eightfish-example-todo/blob/master/backend/schema/create.sql)

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
