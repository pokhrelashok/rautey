# Rautey

Rautey is a little backend framework for building HTTP servers in rust. Rautey has support for route groups, cookies, middlewares, storage and sessions.

## Hello World Example

Here is a basic "Hello World" backend application using the Rautey framework:

```rust
use rautey::{server::Server,request::Request,response::Response};

async fn hello(_:Request,mut res: Responmse,_: HashMap<String,String>) {
  res.text("Hello world");
}

async fn main() {
  let mut server = Server::new("3000");
  server.router.get("/",hello,None);
  server.listen();
}
```

This code sets up a simple web server that responds with "Hello, world!" when accessed at the root URL.

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
