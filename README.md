# Caph

## Projects

- **connector**      -> Wraps authentication, SDE and EVE-API in one project and exposes them as a single library
- **server**         -> Contains the API for the web application
- **web**            -> Web-Application
- **evemon_to_json** -> Converts a evemon file to a json file TODO: rename, rewrite or delete

## Deployment

TODO

### Collector and Server

- Setup Postgres, tested version is 13.x
- Execute the sql script in `./sql/tables.sql`
- Run `make musl` to compile the collector and server binaries
- Create a folder `/opt/caph/` and copy the `target/x86_64-unknown-linux-musl/release/caph_collector` and `target/x86_64-unknown-linux-musl/release/caph_server` into the folder
- Copy `./collector/systemd.service` and `./server/systemd.service` into `/usr/lib/systemd/system` -> rename the files to something unique
- In both systemd files there are env variables that need to be set

#### Server ENV variables

* `SERVER_BIND_ADDR`      -> Address the server should bind to.
                             Default: `127.0.0.1:8080`
* `COLLECTOR_ADDR`        -> Address the collector listen. Optional
* `DATABASE_URL`          -> Database connection string
* `EVE_USER_AGENT`        -> User agent that is set with every request to the EVE-API
* `EVE_CALLBACK`          -> Callback after login, set when creating the application
* `EVE_CLIENT_ID`         -> Client ID provided by EVE when creating the application
* `EVE_SECRET_KEY`        -> Client ID provided by EVE when creating the application
* `EVEPRAISAL_USER_AGENT` -> User-Agent for contacting `https://evepraisal.com/`.
                             More information under [Evepraisal API](https://evepraisal.com/api-docs)
* `JANICE_USER_AGENT`     -> User-Agent for contacting `https://janice.e-351.com/`, this value is optional unless `JANICE_API_KEY` is set.
                             More information under [Janice API](https://janice.e-351.com/api/rest/docs/index.html)
* `JANICE_USER_AGENT`     -> Optional, API-Key for `https://janice.e-351.com`.
                             If the API-Key is set, janice will be prefered over evepraisal
* `REDIRECT`              -> Redirect after the user logged in.
                             Default: `http://localhost:8080`

### Web

TODO

## Development

TODO
