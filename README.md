# Rautey

Rautey is a little backend framework for building HTTP servers in rust. Rautey has support for route groups, cookies, middlewares, storage and sessions.

## Hello World Example

Here is a basic "Hello World" backend application using the Rautey framework:

```rust
use dotenvy::var;
use rautey::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server.router.get(
        "/",
        |_: Request, mut res: Response| {
            res.text("Hello World");
        },
        None,
    );
    server.listen().expect("Could not bind port");
}

```

## Features

### 1. Route Params

```rust
    server.router.get(
        "/users/{id}",
        |req: Request, mut r: Response| {
            r.text(format!(
                "User id is {}",
                req.route_params.get("id").unwrap()
            ));
        },
    );
```

### 2. Route Groups

```rust
    server.router.group("/api", |router: &mut Router| {
        router.get("/", handle_api_home, None);
        router.group("/v2", |router: &mut Router| {
            router.get("/", handle_v2_home, None);
        });
    });
```

### 3. Middlewares

```rust
    server
        .router
        .register_middleware("is-logged-in", |req: &Request, res: &mut Response| {
            if !req.cookies.contains_key("auth_token") {
                res.with_status(rautey::HTTPStatus::UNAUTHORIZED)
                    .text("Permission denied");
            }
        });

    server
        .router
        .get("/users/{id}", get_user_details)
        .with_middlewares(["is-logged-in"]);
```

### 4. Cookies

```rust
    server.router.get("/", |req: Request, mut r: Response| {
        println!("{:?}", req.cookies); //cookies in request
        // add cookies to response
        r.with_cookie(Cookie::new("visited", "yes"))
            .text("Cookies example");
    });
```

### 5. Session

Use `File` or `Cookie` based session by updating `SESSION_DRIVER=file` or `SESSION_DRIVER=cookie`

```rust
    server
        .router
        .get("/", |mut req: Request, mut res: Response| {
            println!(
                "{:?}",
                req.session.get::<String>("session_id").unwrap_or_default()
            );
            req.session.set("session_id", "1234", &mut res);
            res.text("Session example");
        });
```

## Examples

You can find various examples in the `examples` directory. Each example demonstrates different features and use cases of the Rautey framework.

## Running the Example

To run the basic example, use the following command:

```bash
cargo run --example basic
```

This will start the development server. Open your browser and navigate to `http://localhost:8090` to see the example in action.

## Configuration

You may need to configure environment variables. Create a `.env` file in the root directory and add the necessary variables:

```env
# .env
APP_PORT=8090
APP_ENV=local
APP_NAME=basic
APP_PUBLIC_DIR=public

SESSION_DRIVER=cookie
```

## Directories

Make sure the following directories exist in the root:

- `public/`
- `sessions/`
