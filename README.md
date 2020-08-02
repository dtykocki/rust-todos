# Rust Todos

Rust Todos is an overly simplied API for managing todo lists. The primary goal with this project is to gain familarity with [Rocket](https://rocket.rs/) and also to further learnings with Rust. This is my first "real" Rust application so it likely does not follow conventions and needs further improvements. Feedback is welcome and appreciated.

## Getting Started

### Prerequisites

1. Install [Docker](https://docs.docker.com/get-docker/).
2. To run the benchmarks, install [Drill](https://github.com/fcsonline/drill).

### Commands

Start the application:
```bash
make up
```

Stop the application:
```bash
make stop
```

View the logs:
```bash
make logs
```

## API

### List Todos

```bash
curl -X GET http://localhost:4000/todos
```

### Create Todo

```bash
curl \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{
        "name": "Work",
        "completed": false
    }' \
    http://localhost:4000/todos
```

### Show Todo

```bash
curl -X GET http://localhost:4000/todos/5
```

### Update Todo

```bash
curl \
    -X PUT \
    -H "Content-Type: application/json" \
    -d '{
        "name": "Work",
        "completed": true
    }' \
    http://localhost:4000/todos/5
```

### Delete Todo

```bash
curl -X DELETE http://localhost:4000/todos/5
```

## Benchmarking

```bash
drill --benchmark benchmark.yml --stats
```

## Further Improvements
1. Finish the `index` implementation.
2. Add tests.
3. Determine if there is a better way to integrate StatsD telemetry.
4. Benchmark all API calls.
5. Instead of depending on the in memory store, use a real database.
6. Add Authentiation and Authorization.
7. Proper `SIGTERM` handling with Rocket.