# lightning-nodes-service

A Rust service that periodically imports Lightning Network node data from the Mempool API, stores the latest snapshot in PostgreSQL, and exposes it through a JSON API.

## Build tools & versions used

- Rust 1.98.0 / edition 2024
- Axum
- Tokio
- SQLx
- PostgreSQL 18
- Reqwest
- Docker / Docker Compose
- Taskfile
- Test Context
- Testcontainers
- Wiremock

## Architecture

The project uses a small feature-oriented vertical slice.

```text
src/
└── gateways/
    └── mempool/
        ├── fetch_rankings_connectivity.rs
        └── gateway.rs
└── infras/
    ├── database.rs
    ├── env.rs
    └── test_context.rs
└── nodes/
    └── apis/
        └── list.rs
    └── features/
        ├── list.rs
        └── replace.rs
    └── jobs/
        └── job_replace_nodes.rs
    ├── model.rs
├── config.rs
├── errors.rs
├── lib.rs
├── main.rs
├── routes.rs
└── state.rs
```

### System flow

```text
                APPLICATION START
                        │
                        ▼
                app_config::from_env()
                        │
                        ▼
                app_state::create()
                    │         │
                    │         ├── get connection database
                    │         │    └── run migrations
                    │         └── create Gateways
                    │              └── MempoolGateway
                    │
                    ▼
                Arc<AppState>
                │           │
        ┌───────┘           └─────────────┐
        ▼                                 ▼
Background job get nodes                  Axum Router
    every 15 min                          │
        │                                 │
        ▼                                 ▼
  feature replace_nodes            HTTP GET /nodes
get Nodes from MempoolGateway             │
        │                                 ▼
        │                          feature list_nodes
                                get Nodes from database
        │                                 │
        ▼                                 │
    save in database                      ▼
  Node::replace(nodes)              Node::list()
        │                                 │
        ▼                                 │
    postgres database ◀───────────────────
```

## Steps to run the app

### Prerequisites

The application can be run locally using Docker Compose. No local PostgreSQL installation is required. Make sure Docker, Docker Compose, and Taskfile are available.

1. Clone the repository

```bash
git clone <repository-url>
cd lightning-nodes-service
```

2. Configure the environment

```bash
task setup
```

Adjust the values in `.env` if necessary. **Also adjust the corresponding values in `docker-compose.yml`.**

### Running the app

3. Start the application

```bash
task start
```

The application starts the PostgreSQL database, runs the database migrations, starts the background node replacement job, and exposes the HTTP API.

The API is available at: `http://localhost:3000`

4. List Lightning nodes

```bash
curl http://localhost:3000/nodes
```

The endpoint returns the nodes currently stored in the database.

### Running tests

The test suite uses Testcontainers, so PostgreSQL is automatically started by the tests. No PostgreSQL container needs to be manually started for the test suite.

```bash
task test
```

### Run the application locally without Docker

**To run locally, you need to comment out the app service in `docker-compose.yml`.** Keep only the PostgreSQL service to connect to the database.

```bash
task local/run
```

The application will run the database migrations automatically during startup.

### Run check formatting and clippy

```bash
task check-all
```

## What was the reason for your focus? What problems were you trying to solve?

The main focus was to build a small but production-oriented service for retrieving and serving Lightning Network node data.

The service has two main responsibilities:

1. Periodically retrieve the latest node information from the Mempool API.
2. Expose the stored snapshot through an HTTP API.

I focused on keeping the implementation simple and easy to reason about while still addressing some important backend concerns:

- Clear separation between external API communication, business features, persistence, and HTTP handling.
- Automatic database migrations during application startup.
- Periodic synchronization of node data at a configurable interval — just adjust the env variable.
- Atomic replacement of the database snapshot using a PostgreSQL transaction.
- Proper propagation and categorization of gateway and database errors.
- HTTP error responses with stable error codes and request IDs.
- Integration tests using a real PostgreSQL instance through Testcontainers.
- Gateway tests using an HTTP mock instead of depending on the real Mempool service.
- Configuration tests without mutating the developer's environment permanently.

## How long did you spend on this project?

Approximately 12 hours.

The time was primarily spent on the implementation, test coverage, project structure, error handling, and making the service easy to run locally.

## Did you make any trade-offs for this project? What would you have done differently with more time?

Yes.

**Background job:** the node synchronization currently runs as a background Tokio task inside the application process and executes every 15 minutes. This keeps the solution simple and avoids introducing another infrastructure component. With more time, I would consider whether the synchronization job should eventually be moved to a dedicated worker/scheduler depending on the expected scale and deployment model.

**API scope:** the API currently focuses on the required `GET /nodes` use case. With more time, I would consider pagination, filtering, and potentially additional node metadata depending on the expected consumers.

## What do you think is the weakest part of your project?

The integration tests require PostgreSQL. A CI pipeline could provision an isolated PostgreSQL service automatically.

The synchronization job is intentionally simple, but it does not provide distributed coordination. If multiple instances of the service were deployed, each instance would execute the 15-minute synchronization independently.

## API Docs

### GET /nodes

Example:

```json
[
  {
    "public_key": "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f",
    "alias": "ACINQ",
    "capacity": "360.10516297",
    "first_seen": "2018-04-05T15:13:42Z"
  }
]
```
