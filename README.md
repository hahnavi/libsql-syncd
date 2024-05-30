# libsql-syncd

**libsql-syncd** is a tool that synchronizes your libSQL embedded replica with a primary database using Redis Pub/Sub to trigger synchronization. This keeps your embedded replica up-to-date efficiently and provides an easy way to maintain synchronization without the need for constant polling.

## Getting Started

To run **libsql-syncd**, follow these steps:

1. Build the Project:

    ```sh
    cargo build --release
    ```

1. Set Environment Variables:

    If you prefer, you can set environment variables manually or use `.env` file.
    - Copy the `.env.example` file to a new file named `.env`.
    - Edit the `.env` file and replace placeholders with your actual values.
      - `LIBSQL_SYNC_URL`: URL of the primary.
      - `LIBSQL_AUTH_TOKEN`: Authentication token.
      - `LIBSQL_DB_PATH`: Location of the embedded replica file.
      - `REDIS_URL`: URL of the Redis server.
      - `REDIS_CHANNEL`: Channel name of the Redis Pub/Sub.
      - `HOST_ID`: ID of the host. If the value matches the message from Redis Pub/Sub, sync will be skipped.
  
    

1. Run the App:

    - After building the project, navigate to the `target/release` directory.
    - Run the generated binary file:

        ```sh
        ./libsql-syncd
        ```

