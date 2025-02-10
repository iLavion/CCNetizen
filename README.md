# CCNetizen

## Prerequisites

- [Docker](https://www.docker.com/get-started) (or [Docker Desktop](https://www.docker.com/products/docker-desktop) for Windows/Mac)
- Rust and Cargo (Install from [rustup.rs](https://rustup.rs/))

## Setup

### Step 1: Clone the Repository

```sh
git clone https://github.com/yourusername/CCNetizen.git
cd CCNetizen
```

### Step 2: Start LocalStack

LocalStack is a fully functional local AWS cloud stack. We will use it to simulate AWS services locally.

#### Using Docker Compose

If you have Docker installed, you can start LocalStack using the provided `docker-compose.yml` file.

```sh
docker-compose up
```

#### Without Docker

If you don't have Docker installed, please visit the [Docker installation page](https://www.docker.com/get-started) and follow the instructions to install Docker or Docker Desktop.

### Step 3: Set Environment Variables

Set the environment variable to run the application in development mode.

#### Windows

```sh
set APP_ENV=development
```

#### Linux/Mac

```sh
export APP_ENV=development
```

### Step 4: Run the Application

Use Cargo to run the application.

```sh
cargo run
```

## Configuration

The application uses a `config.toml` file for configuration. Ensure you have the correct AWS credentials and endpoints set up in this file.

```toml
# config.toml
aws_access = "your_access_key"
aws_secret = "your_secret_key"
aws_region = "your_region"
aws_endpoint = "http://localhost:4566"

aws_access_dev = "your_access_key"
aws_secret_dev = "your_secret_key"
aws_region_dev = "your_region"
aws_endpoint_dev = "http://localhost:4566"
```

## Usage

The application is designed to fetch data from a specified URL, process the data to extract information about towns, and store this information in a local DynamoDB instance provided by LocalStack. The data fetching runs in a continuous loop with a delay between each fetch.

### Fetching Data

The application fetches data from a URL that provides information about towns. This data is in JSON format and contains various details about each town, such as the name, mayor, residents, resources, and more.

### Processing Data

Once the data is fetched, the application processes it to extract relevant information about each town. This includes parsing the JSON data, extracting specific fields, and performing any necessary transformations.

### Storing Data

After processing the data, the application stores the information in a local DynamoDB instance. This allows for efficient querying and retrieval of town information. The data is stored with a timestamp to keep track of the last update.

### Querying Data

The application provides functionality to query the stored data. This includes retrieving information about a specific town by its name. The querying is case-insensitive, ensuring that town names can be queried regardless of their case.

### Example Workflow

1. The application starts and sets up the necessary environment.
2. It enters a loop where it fetches data from the specified URL at regular intervals.
3. The fetched data is processed to extract information about towns.
4. The processed data is stored in the local DynamoDB instance.
5. The application continues to fetch, process, and store data in a loop.

### Error Handling

The application includes error handling to manage issues that may arise during data fetching, processing, or storing. Errors are logged, and the application continues to run, ensuring that temporary issues do not cause the application to stop.

## Troubleshooting

If you encounter any issues, ensure that:

- Docker is running and LocalStack is started.
- The `APP_ENV` environment variable is set correctly.
- The `config.toml` file is configured with the correct AWS credentials and endpoints.