# Caph

## Projects

- **collector** -> Collects data either from the SDE.zip file, from the EVE-API or from other external APIs
- **connector** -> Wraps authentication, SDE and EVE-API in one project and exposes them as a single library
- **server** -> Deprecated as soon as `server_v2` is done
- **server_v2** -> Contains the API for the web application
- **web** -> Web-Application
- **evemon_to_json** -> Converts a evemon file to a json file TODO: rename, rewrite or delete

## Deployment

### Collector and Server

- Setup Postgres, tested version is 13.x
- Execute the sql script in `./sql/tables.sql`
- Run `make musl` to compile the collector and server binaries
- Create a folder `/opt/caph/` and copy the `target/x86_64-unknown-linux-musl/release/caph_collector` and `target/x86_64-unknown-linux-musl/release/caph_server` into the folder
- Copy `./collector/systemd.service` and `./server/systemd.service` into `/usr/lib/systemd/system` -> rename the files to something unique
- In both systemd files there are env variables that need to be set

#### Collector ENV variables

* `COLLECTOR_BIND_ADDR` -> Address the server should bind to.
                           Default: `127.0.0.1:9090`
* `DATABASE_URL` -> Database connection string
* `EVE_USER_AGENT` -> User agent that is set with every request to the EVE-API
* `EVE_CLIENT_ID` -> Client ID provided by EVE when creating the application
* `EVE_SECRET_KEY` -> Client ID provided by EVE when creating the application

#### Server ENV variables

* `SERVER_BIND_ADDR` -> Address the server should bind to.
                        Default: `127.0.0.1:8080`
* `DATABASE_URL`     -> Database connection string
* `EVE_USER_AGENT`   -> User agent that is set with every request to the EVE-API
* `EVE_CALLBACK`     -> Callback after login, set when creating the application
* `EVE_CLIENT_ID`    -> Client ID provided by EVE when creating the application
* `EVE_SECRET_KEY`   -> Client ID provided by EVE when creating the application
* `REDIRECT`         -> Redirect after the user logged in.
                        Default: `http://localhost:8080`

### Web

TODO
