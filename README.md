# Simple REST web server
### Based on the final project from the Rust docs.


<br /><br />

So far includes:
- JSON
- Web sockets
- Postgres

<br /><br />


### DB

Install postgres on your OS.<br />
Install PgAdmin for better expirience.<br />
Use `test_user_pg` user to connect to PG.<br />
Create `test_db` database.<br />
Create `person` table with the only `string` column `name`.<br />
All the above params, like user or database are configurable in `src/bin/main`.<br />
Start a postgresql server.<br /><br /><br />


### Server
Install Rust on your system.<br />
To start the server - from the root dir run `cargo run`.<br /><br /><br />

### Client

Install `siegel` package globally: `npm i -g siegel`.<br />
To start a demo client - from the root dir run `siegel run -g --client client.js`.
