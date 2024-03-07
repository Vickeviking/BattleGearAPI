
## Commands

### Start up container 

docker-compose up   //starts container
docker-compose exec app cargo run //opens up window of backend view
docker-compose exec app diesel migration list // se if migrations are up
docker-compose exec app diesel migration run // run if not up
// check with list that everything is up

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

## run commands in postgres
docker ps #find id of postgres-latest
docker exec -it <container-name-or-id> psql -U postgres -d app_db
To exit psql, use the \q command.

## use cli
run container
make sure migrations are up
docker-compose exec app cargo run --bin cli 



## Instructions

1. Create a PostgreSQL database.
2. Execute the SQL `CREATE TABLE` statements provided in `database_setup.sql` to create the necessary tables.
3. (Add any additional instructions for setting up the database, such as populating initial data or setting up indexes and constraints)

## Usage

(Provide instructions on how to interact with the database, such as querying data, updating records, etc.)

## Contributors

- [Your Name](link to your GitHub profile)
- (Add other contributors if applicable)

## License

This project is licensed under the [MIT License](link to license file).



## Updating and Deploying Docker Image to Raspberry Pi

### Step 1 - build for platform
docker buildx build --load -t armv7_battlegearapi:latest --progress plain --platform linux/arm/v7 .

### Step 2 - save to .tar
docker save armv7_battlegearapi:latest > armv7_battlegearapi.tar

### Step 3 - send to rasperryPi
scp armv7_battlegearapi.tar vango@192.168.0.215:docker_images

### Step 4 - load .tar into docker
docker load < armv7_battlegearapi.tar

### Step 5 - Make a docker-compose ONLY ONCE
echo "version: '3'
services:
  app:
    image: armv7_battlegearapi:latest
    ports:
      - '8080:8080'" > docker-compose.yml

### Step 6 - Run 
docker-compose up -d

# BattleGearAPI
