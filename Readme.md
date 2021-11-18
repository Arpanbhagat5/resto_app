
# Overview

Create a restaurant application which accepts menu items from various serving staff in the restaurant. This application must then store the item along with a countdown for the item to be ready to serve. The application must be able to give a quick snapshot of any or all items on its list at any time. It must also be able to remove specific orders from the list of orders on demand.
</br></br>


# Structure of data
Created a relational normalized DB for this application. Below is the ERD (might need some updates)
![DB structure](/resto_app_db.png)
</br></br>


# Expected features
- [x] The client MUST be able to:
  - add one or more items with a table number
    - Done (one item at a time)
  - remove an item for a table
    - Done
  - query the items still remaining for a table.
    - Done

```
# Create order for table
$ curl -X POST -d '{"table_id": 1}' http://localhost:8000/api//table/create_order

{"order_id":4,"table_id":1,"created_at":"2021-11-15T15:37:11.953926Z"}


# Remove order for table (SERVE)
$ curl -X POST http://localhost:8000/api/table/2/serve_item/2

[{"id":9,"item_id":2,"order_id":3,"item_status_id":2,"prep_time":0,"created_at":"2021-11-16T10:31:28.987088Z"}]

$ curl -X GET http://localhost:8000/api/table/1/remaining_items

[{"id":2,"item_id":1,"order_id":1,"item_status_id":2,"prep_time":10,"created_at":"2021-11-16T08:58:06.275437Z"}]

```

- [x] The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
  - Done

```
$ curl -X POST -d '{"item_id": 1, "order_id": 1, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item

{"id":2,"item_id":1,"order_id":1,"item_status_id":2,"prep_time":10,"created_at":"2021-11-16T08:58:06.275437Z"}
```

- [x] The application MUST, upon deletion request, remove(CANCEL) a specified item for a specified table number.
  - Done

```
# single item
$ curl -X POST http://localhost:8000/api/table/2/cancel_item/5

[{"id":13,"item_id":5,"order_id":3,"item_status_id":3,"prep_time":10,"created_at":"2021-11-16T10:32:06.550488Z"}]

# Multiple instance of same item
$ curl -X POST http://localhost:8000/api/table/2/cancel_item/9

# Remaining items of a table's order
[{"id":21,"item_id":9,"order_id":3,"item_status_id":3,"prep_time":5,"created_at":"2021-11-17T10:35:10.389963Z"},
{"id":25,"item_id":9,"order_id":3,"item_status_id":3,"prep_time":5,"created_at":"2021-11-17T10:35:10.431710Z"},
{"id":26,"item_id":9,"order_id":3,"item_status_id":3,"prep_time":10,"created_at":"2021-11-17T10:35:10.434738Z"}]

```

- [x] The application MUST, upon query request, show all items for a specified table number.
  - Done

```
$ curl -X GET http://localhost:8000/api/table/1/all_items

[{"id":2,"item_id":1,"order_id":1,"item_status_id":2,"prep_time":10,"created_at":"2021-11-16T08:58:06.275437Z"}]
```

- [x] The application MUST, upon query request, show a specified item for a specified table number.
  - Done

```
# If only one instance is in order
$ curl -X GET http://localhost:8000/api/table/3/get_item/7

[{"id":32,"item_id":7,"order_id":4,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.436101Z"}]

# If multiple instance os same item are in order
curl -X GET http://localhost:8000/api/table/2/get_item/5

[{"id":10,"item_id":5,"order_id":3,"item_status_id":3,"prep_time":7,"created_at":"2021-11-16T10:31:35.853010Z"},
{"id":11,"item_id":5,"order_id":3,"item_status_id":3,"prep_time":10,"created_at":"2021-11-16T10:31:42.004046Z"},
{"id":13,"item_id":5,"order_id":3,"item_status_id":3,"prep_time":10,"created_at":"2021-11-16T10:32:06.550488Z"}
```

- [x] The application MUST accept at least 10 simultaneous incoming add/remove/query requests.
  - Done
    - Tested this simply with async way of doing curl requests. Below is one such example

```
curl -X POST -d '{"item_id": 1, "order_id": 1, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 1, "order_id": 1, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 2, "order_id": 1, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 3, "order_id": 1, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 4, "order_id": 1, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 5, "order_id": 1, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 7, "order_id": 2, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 8, "order_id": 2, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 9, "order_id": 2, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 9, "order_id": 3, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 9, "order_id": 3, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 9, "order_id": 3, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 1, "order_id": 4, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 3, "order_id": 4, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 5, "order_id": 4, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 5, "order_id": 4, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 7, "order_id": 4, "prep_time": 15, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 1, "order_id": 5, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 2, "order_id": 5, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 3, "order_id": 5, "prep_time": 10, "item_status_id": 1}' http://localhost:8000/api/table/add_item & \
curl -X POST -d '{"item_id": 10, "order_id": 5, "prep_time": 5, "item_status_id": 1}' http://localhost:8000/api/table/add_item

[1] 39863
[2] 39864
[3] 39865
[4] 39866
[5] 39867
[6] 39868
[7] 39869
[8] 39870
[9] 39871
[10] 39872
[11] 39873
[12] 39874
[13] 39875
[14] 39876
[15] 39877
[16] 39878
[17] 39879
[18] 39880
[19] 39881
[20] 39882
{"id":23,"item_id":5,"order_id":1,"item_status_id":1,"prep_time":5,"created_at":"2021-11-17T10:35:10.387898Z"}{"id":17,"item_id":1,"order_id":1,"item_status_id":1,"prep_time":5,"created_at":"2021-11-17T10:35:10.388287Z"}{"id":19,"item_id":9,"order_id":2,"item_status_id":1,"prep_time":5,"created_at":"2021-11-17T10:35:10.389957Z"}
[6]    39868 done       curl -X POST -d  http://localhost:8000/api/table/add_item
{"id":24,"item_id":2,"order_id":1,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.388134Z"}{"id":16,"item_id":1,"order_id":1,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.387541Z"}
[9]    39871 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[2]    39864 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[1]    39863 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[3]    39865 done       curl -X POST -d  http://localhost:8000/api/table/add_item
{"id":21,"item_id":9,"order_id":3,"item_status_id":1,"prep_time":5,"created_at":"2021-11-17T10:35:10.389963Z"}{"id":15,"item_id":7,"order_id":2,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.387552Z"}{"id":25,"item_id":9,"order_id":3,"item_status_id":1,"prep_time":5,"created_at":"2021-11-17T10:35:10.431710Z"[10]    39872 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[7]    39869 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[11]    39873 done       curl -X POST -d  http://localhost:8000/api/table/add_item
{"id":22,"item_id":4,"order_id":1,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.388775Z"}{"id":18,"item_id":8,"order_id":2,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.388787Z"}{"id":20,"item_id":3,"order_id":1,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.388060Z"[8]    39870 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[5]    39867 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[4]    39866 done       curl -X POST -d  http://localhost:8000/api/table/add_item
{"id":32,"item_id":7,"order_id":4,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.436101Z"}{"id":27,"item_id":3,"order_id":4,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.434808Z"}
{"id":26,"item_id":9,"order_id":3,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.434738Z"}{"id":28,"item_id":5,"order_id":4,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.435411Z"}{"id":29,"item_id":1,"order_id":4,"item_status_id":1,"prep_time":15,"created_at":"2021-11-17T10:35:10.435750Z"}[17]    39879 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[14]    39876 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[12]    39874 done       curl -X POST -d  http://localhost:8000/api/table/add_item
{"id":30,"item_id":5,"order_id":4,"item_status_id":1,"prep_time":10,"created_at":"2021-11-17T10:35:10.435985Z"}
[16]    39878 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[15]    39877 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[13]    39875 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[18]    39880 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[20]  + 39882 done       curl -X POST -d  http://localhost:8000/api/table/add_item
[19]  + 39881 done       curl -X POST -d  http://localhost:8000/api/table/add_item
```

- [ ] The client MAY limit the number of specific tables in its requests to a finite set (at least 100).
  - Not Implemented

- [x] The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.
  - Done
  - The time is not randomized, but needs to be passed as a parameter

- [x] The application MAY keep the length of time for the item to prepare static (in other words, the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion).
  - Done

```
# Remove order for table (SERVE)
$ curl -X POST http://localhost:8000/api/table/2/serve_item/2
[{"id":9,"item_id":2,"order_id":3,"item_status_id":2,"prep_time":0,"created_at":"2021-11-16T10:31:28.987088Z"}]

```

</br>

# Tech stack
```
$ rustup --version
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.58.0-nightly (e90c5fbbc 2021-11-12)`

$ diesel --version
diesel 1.4.1

$ psql --version
psql (PostgreSQL) 13.3

```
</br>

# App setup
</br>

## OPTION 1: Build and run locally
</br>

### Pre-requisite
- Have executable permission on runnable scripts
  - set_env_var_for_local_run.sh
  - build_run_container.sh

### Setup DB

`$ sudo -u postgres psql`

<!-- In psql console -->
Login to psql console and run the following
```
postgres=# create database resto_app;
postgres=# create user app_user with encrypted password 'password';
postgres=# grant all privileges on database resto_app to app_user;
```
### Setup rocket
```
$ install rustc 1.58.0-nightly
$ rustup --version

rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.58.0-nightly (e90c5fbbc 2021-11-12)`

$ rustup default nightly-2021-11-13
```
  - Rocket requires the nightly compiler
  - Actually rocket-contrib requires it for provisioning JSON handling functionalities
</br>

### Install diesel
`$ cargo install diesel_cli --no-default-features --features "postgres"`

### Setup diesel and migrations
`$ diesel setup`
  - Must run before migration commands
  - Set DATABASE_URL env var (used .env for that)
  - Must have DB
</br>

### Create migration and update up/ down sql scripts
`$ diesel migration generate create_db_tables`
  - Refer migration files for this
</br>

### Run migrations
`$ diesel migration run`
</br>

### Populate schema
`$ diesel print-schema > ./src/schema.rs`
</br>

### Set local env vars
`./set_env_var_for_local_run.sh`
### Check syntax server
`$ cargo check`
</br>

### Run server
`$ cargo run`
</br></br>

## OPTION 2: Run from Docker
</br>

### Pre-requisite
- Have Docker desktop client for Mac/Windows

### Run the docker script from root of the directory
`$ ./build_run_container.sh`

### Check running docker containers
`$ docker ps`

### Check server is up and running
```
$ curl localhost:8000/

Hello, Resto App!
```
</br>

## TODO / DISCLAIMER

### What I enjoyed most
- The sight of seeing zero errors on `$ cargo check` at 2 AM
- Finally, getting hang of the type management of Rust
- Being able to separate out the implementation of API, backend and Db layers
- Knowing that Rust makes sure the code can not break for so many cases that would easily happen for a different language
- Traits that did wonders
- Custom vectors (IDK the correct name maybe) that become the return value for DB functions that run SQL queries directly

### Where I lost a lot of time
- Declarative types of Rust (first attempt and beginning phase)
- Lots and lots of going back and forth to undersatnd the type mismatches and other errors
- Having to restart the development for the second time, only after realizing I need to refactor a lot on [my first attempt](https://github.com/Arpanbhagat5/resto-api)
- The fact that I could have done much better, if I knew this language a bit more and didn't have a very tired soul

### Assumption that backfired
- I took a not so well thought assumption that someone could order multiple instances of same thing
    - this basically makes the cancel and serve(delete) requests slightly inaccurate
    - I had to cancel/serve all instance at once

### Todo
- Implement timer logic for counting down `prep_time` and change status of item from `preparing` -> `served`
   - Handle cancelation request based on current status: Handle/drop running thread in case of cancelation

### No tests :(
- I could not add tests

### Exceptions not handled for corner cases
- Instead of creating a proper JSON reponse, I returned the objects (vector of objects)
- In case of non-existent values from DB, the server gives 5xx error while printing to the stdout the reason
  - This basically doesnt work well with corner cases or when exceptions occur
  - For example, if you add multiple items, and one of the items in not in the item list, the server will give back a 5xx error, while printing to the stdout that the object doent exist.
  - Ideally, I would handle this by implementing a generic Json response struct
