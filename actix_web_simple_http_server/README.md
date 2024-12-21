# Actix Web Server Example

This project demonstrates a simple HTTP server using the [Actix Web](https://actix.rs/) framework in Rust. The server has two primary functionalities:

1. Responding to a greeting request (`/hello/{name}`) with a personalized message.
2. Serving an HTML file on the root route (`/`).

## Features

- **Dynamic Routing:**
  - The server dynamically handles the `/hello/{name}` route and extracts the `name` from the URL path to generate a personalized greeting.
- **Static File Serving:**
  - Serves a static HTML file located at `html/index.html`.
  - If the file is not found, it returns a custom 404 error message.
- **Integration Tests:**
  - Comprehensive tests for the routes to ensure expected functionality.

## File Structure

```
.
├── src
│   └── main.rs        # Main application logic
├── html
│   └── index.html     # Static HTML file served at the root route
└── tests
    └── integration_test.rs  # Integration tests for the application
```

## Usage

### Prerequisites

- Rust (Edition 2021 or later) installed on your machine.

### Running the Server

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```
2. Build and run the server:
   ```bash
   cargo run
   ```
3. Open your browser and navigate to:
   - [http://127.0.0.1:8080/](http://127.0.0.1:8080/) to view the HTML page.
   - [http://127.0.0.1:8080/hello/your_name](http://127.0.0.1:8080/hello/your_name) to receive a personalized greeting.

### Running Tests

To run the integration tests:

```bash
cargo test
```

## Code Explanation

### Main Server Logic

#### Route: `/hello/{name}`
This route is handled by the `greet` function. It extracts the `name` from the URL path and returns a personalized greeting.

```rust
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
```

#### Route: `/`
This route is handled by the `index` function. It reads the `html/index.html` file and serves it to the client. If the file is not found, it returns a custom 404 message.

```rust
async fn index(req: HttpRequest) -> impl Responder {
    let html_content = fs::read_to_string("html/index.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(html_content)
}
```

### Integration Tests

The integration tests validate the server's behavior for both routes.

#### Test: `/hello/{name}`
Verifies that the server responds with the correct greeting message.

```rust
#[actix_web::test]
async fn test_greet() {
    let app = test::init_service(App::new().service(greet)).await;

    let req = test::TestRequest::get().uri("/hello/testuser").to_request();
    let resp = test::call_and_read_body(&app, req).await;
    assert_eq!(resp, "Hello testuser!");
}
```

#### Test: `/`
Checks if the server serves the correct HTML content.

```rust
#[actix_web::test]
async fn test_index() {
    let app = test::init_service(App::new().service(web::resource("/").to(index))).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let _resp = test::call_and_read_body(&app, req).await;
}
```

## Dependencies

- `actix-web` v4

Add the dependency in `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
```

## Contribution

Contributions are welcome! Feel free to submit a pull request or report an issue.

## License

This project is licensed under the GNU Version 3 License.
