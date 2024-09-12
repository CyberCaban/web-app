<p align="center">
    <h1 align="center">Web App</h1>
    <p align="center">
        Web file server with authentication. 
    </p>
</p>

## Made with

- Rust, a systems programming language.
- Diesel, a safe, extensible ORM and query builder.
- Rocket, a web framework for building web applications.
- PostgreSQL, a powerful, open source object-relational database system.
- Docker, a platform for containerizing and deploying applications.
- React, a JavaScript library for building user interfaces.
- Tailwind CSS, a utility-first CSS framework for building custom user interfaces.

## Features

- Authentication (username/password)
- File upload
- File download
- File deletion

<!-- </br>The frontend is written in React and uses Tailwind CSS for styling.
</br>The backend is written in Rust and uses Diesel for database operations and Rocket for the web server.
</br>The project uses PostgreSQL as the database.
</br>The project uses Docker to build and run the application.
</br>The project uses HTTP cookies to store the authentication token. -->

## Usage

Prerequisites:

- [PostgreSQL](https://www.postgresql.org) (libpq-dev)
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/)
- [Docker](https://www.docker.com) (optional)

### With Docker

```bash
docker build -t web-app .
docker run -p 8000:8000 web-app -e DATABASE_URL=`your_database_url`
```

### Without Docker

Build frontend inside the `www` folder:

```bash
cd www
npm install
npm run build
```

Start the server:

```bash
cargo run --release
```

## References

- [Docker](https://www.docker.com)
- [Rust](https://www.rust-lang.org)
- [Node.js](https://nodejs.org/en/)
- [PostgreSQL](https://www.postgresql.org)
- [Rocket](https://rocket.rs)
- [Diesel](https://diesel.rs)
- [Tailwind CSS](https://tailwindcss.com)
- [React](https://reactjs.org)
- [Vite](https://vitejs.dev)
