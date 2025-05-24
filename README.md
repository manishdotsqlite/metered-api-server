### METERED API WEB SERVER USING WARP

This is a simple web server built using the Warp framework in Rust. It provides an API for addition. This server is rate-limited to allow only 50 requests per hour. If limit is exceeded, it returns an error message.

### Features

- Simple addition API
- Rate limiting (50 requests per hour)
- Error handling for rate limit exceeded

### Getting Started

1. Clone the repository:
   `bash
        git clone https://github.com/manishdotsqlite/metered-api-server.git
`

2. Navigate to the project directory:
   `bash
       cd metered-api-server
`

3. Run the server:
   `bash
            cargo watch -q -c -w src/ -x run
    `

### API Endpoints

# User validation endpoint

- `GET http://127.0.0.1:3031/validate/{username}`

  - **Description**: Validates the user by checking if the username exists in the database.
  - **Response**: Returns a JSON object with a success message if the user is valid, otherwise an error message.
  - **Example**:
    ```json
    {
      "status-code": 200,
      "message": "User is valid",
      "data": "username"
    }
    ```

# API generation endpoint

- `GET http://127.0.0.1:3031/create-api-key/{username}`

  - **Description**: Creates an API key for the user.
  - **Response**: Returns a JSON object with the API key if successful, otherwise an error message.
  - **Example**:
    ```json
    {
      "status-code": 200,
      "message": "API key created successfully",
      "data": "api_key"
    }
    ```

# API deletion endpoint

- `POST http://127.0.0.1:3031/delete-api-key/{username}/{api_key}`

  - **Description**: Deletes the API key for the user.
  - **Response**: Returns a JSON object with a success message if the API key is deleted, otherwise an error message.
  - **Example**:
    ```json
    {
      "status-code": 200,
      "message": "API key deleted successfully",
      "data": "None"
    }
    ```

# Perform addition endpoint

- `GET http://127.0.0.1:3031/add/{a}/{b}/{username}/{api_key}`

  - **Description**: Performs addition of two numbers.
  - **Response**: Returns a JSON object with the result of the addition.
  - **Example**:
    ```json
    {
      "status-code": 200,
      "message": "Addition successful",
      "data": 5
    }
    ```

### Rate Limiting

The server is rate-limited to allow only 50 requests per hour. If the limit is exceeded, it returns an error message:

```json
{
  "status-code": 429,
  "message": "Rate limit exceeded. Please try again later.",
  "data": null
}
```
