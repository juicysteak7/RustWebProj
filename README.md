# RustWebProj
This project is built to track job applications.
Ensure you have rust and cargo installed before following further instructions.

[Rust Install](https://www.rust-lang.org/tools/install)

# Front-end
The front end uses Yew (similar to react) to build web components that compile to web assembly.
Trunk is a cargo package that can deploy to multiple targets, for this application I am targeting a web application.

To install trunk run the command
```
cargo install trunk
```
To set web assembly as a build target run the command
```
rustup target add wasm32-unknown-unknown
```
To run the front end once dependencies are installed use the following command.
```
trunk serve
```

# Back-end
The backend of this project uses tokio and tower to build api responses in a async environment. Dependencies in the backend should install on their own.

To build the backend use the following command
```
cargo build
```

To run the backend use the following command
```
cargo run
```

# Database
The database this project uses is SurrealDB

[SurrealDB Install](https://surrealdb.com/start)

Once surrealdb is installed you can use the following command to start the database
```
surreal start --user root --pass root rocksdb://../applications.db 
```

