# Rautey

Rautey is a web framework for building fast and secure web applications with Rust.

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
