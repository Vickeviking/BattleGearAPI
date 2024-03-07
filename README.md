
## Commands

### Start up container 

1. Start the container:
    ```bash
    docker-compose up
    ```
2. Open up the window of backend view:
    ```bash
    docker-compose exec app cargo run
    ```
3. Check if migrations are up:
    ```bash
    docker-compose exec app diesel migration list
    ```
4. If migrations are not up, run them:
    ```bash
    docker-compose exec app diesel migration run
    ```
5. Check again to ensure that everything is up:
    ```bash
    docker-compose exec app diesel migration list
    ```

### test
1. Start the container:
    ```bash
    docker-compose up
    ```
2. Open up the window of backend view:
    ```bash
    docker-compose exec app cargo run
    ```
3. Check if migrations are up:
    ```bash
    docker-compose exec app diesel migration list
    ```
4. Run if not up:
    ```bash
    docker-compose exec app diesel migration run
    ```
5. Run tests:
    ```bash
    docker-compose exec app cargo test
    ```

## Run commands in Postgres

1. Find the ID of `postgres-latest`:
    ```bash
    docker ps
    ```
2. Run `psql` in the Postgres container:
    ```bash
    docker exec -it <container-name-or-id> psql -U postgres -d app_db
    ```
    To exit `psql`, use the `\q` command.

## Use CLI

1. Run the container:
    ```bash
    docker-compose up
    ```
2. Make sure migrations are up:
    ```bash
    docker-compose exec app diesel migration list
    docker-compose exec app diesel migration run
    ```
3. Run the CLI:
    ```bash
    docker-compose exec app cargo run --bin cli
    ```

## Contributors

- Viktor liljenberg, https://github.com/Vickeviking

## License

This project is licensed under the [MIT License](link to license file).


## Updating and Deploying Docker Image to Raspberry Pi

Build the whole with docker build 
```bash
    docker build
```

6. **Run:**
    ```bash
    docker-compose up -d
    ```
