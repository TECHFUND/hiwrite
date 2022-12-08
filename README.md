

## Getting Started

To get started with our CMS written in Rust, you will need to have the following installed on your system:

    Rust: https://www.rust-lang.org/tools/install
    Postgres: https://www.postgresql.org/download/

Once you have these dependencies installed, you can follow these steps to get the CMS up and running:

Clone this repository to your local machine

    git clone https://github.com/TECHFUND/hiwrite

Navigate to the project directory and build the project

      cd rust-cms
      cargo build

Create a .env file in the project root and set the following environment variables:

    DATABASE_URL=postgres://[username]:[password]@localhost/rust_cms

Migrate the database

    cargo run -- migrate

Start the server

    cargo run -- server

The CMS should now be up and running at `http://localhost:8000`

## API

The CMS exposes a RESTful API that allows you to manage the content of your site. 

The API documentation can be found at http://localhost:8000/api/docs.
License

This project is licensed under the gplv3 License. See LICENSE for more details.
