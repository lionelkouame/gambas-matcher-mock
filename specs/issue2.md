> ## 🎫 Hexagonal Scaffold Architecture & Project Boilerplate
> **Description:** Initialize the folder structure and module system to adhere to the principles of the hexagonal architecture. This step ensures that the "Domain" (Core business logic) remains completely independent of the technical details (Infrastructure).
>
> ### 📋 Checklist / Tasks
> * [x] 
    >   [x]  **Cargo Initialization**: Configure the `Cargo.toml` file with the basic dependencies (Tokio, SQLx, Serde, Chrono, UUID).
    >   [x]
    >   [x]  **Folder Tree**: Create the `src/domain`, `src/application`, and `src/infrastructure` hierarchies.
    >   [x]
    >   [x]  **Module System**: Configure the `mod.rs` files for selectively exposing components.
    >   [x]
    >   [x]  **Domain Database**: Create the `src/domain/entities.rs` file for the `Project` and `MockConfig` entities.
    >   [x]
    >   [x]  **Hello Word** for test the  architecture
>
> ### 📂 Expected File Structure
> ```
> src/
> ├── main.rs # Bootstrap & DI (Dependency Injection)
> ├── domain/ # Core Business (Pure Rust)
> │ ├── mod.rs
> │ └── entities.rs # Project & MockRule
> ├── application/ # Use Cases
> │ ├── mod.rs
> │ └── services/ # CRUD Management Logic
> ├── infrastructure/ # Adapters (Technical Details)
> │ ├── mod.rs
> │ ├── persistence/ # SQLite (SQLx) Implementation
> │ └── web/ # Interface (Axum)
> └── shared/ # Common errors and utilities 
> └── mod.rs
> ```
>
> ### ⚙️ Configuration `Cargo.toml` (Initial)
> [package]
> name = "gambas-matcher-mock"
> version = "0.1.0"
> edition = "2021"
>
> [dependencies]
> # Runtime & Async
> tokio = { version = "1", features = ["full"] }
>
> # Data & Serialization
> serde = { version = "1.0", features = ["derive"] }
> serde_json = "1.0"
> chrono = { version = "0.4", features = ["serde"] }
> uuid = { version = "1.0", features = ["v4", "serde"] }
>
> # Database
> sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros", "chrono"] }
>
> # Error Handling
> thiserror = "1.0"
> ### 🛠️ Technical Instructions for the Developer
> 1. **Domain Isolation**: The `src/domain` folder must not import any external crates related to the network or database (no `sqlx`, no `axum`).
> 2. **Visibility**: Use `pub(crate)` for internal functions and `pub` only for what needs to be exposed to other layers.
> 3. **Bootstrap**: The `main.rs` file will be responsible for instantiating the SQLite connection and injecting it into the application's services.
