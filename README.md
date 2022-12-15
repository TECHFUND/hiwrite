

## Getting Started

To get started with our CMS written in Rust, you will need to have the following installed on your system:

    Rust: https://www.rust-lang.org/tools/install
    Postgres: https://www.postgresql.org/download/

The easiet way to install Postgres is via https://postgresapp.com/ app. Follow the following instrructions to install Postgres app.



1) Download the app from https://postgresapp.com/downloads.html and Move it to Applications folder, Double Click to open the application
   If you don't move Postgres.app to the Applications folder, you will see a warning about an unidentified developer and won't be able to open it.
2) Click "Initialize" to create a new server and make sure you are using PostgreSQL 14
3) Prepare the $PATH to include postgres in ENV

        sudo mkdir -p /etc/paths.d &&
        echo /Applications/Postgres.app/Contents/Versions/latest/bin | sudo tee /etc/paths.d/postgresapp

<img width="299" alt="postgres" src="https://user-images.githubusercontent.com/4836107/207767101-3ed0b7bb-7306-49de-a4dc-3de98ca07b8a.png">

Once you have Postgres 14 up and running , prepare a new table called "hiwrite"

<img width="810" alt="DB" src="https://user-images.githubusercontent.com/4836107/207767467-cd6d6753-9c4f-4ee6-afd1-01d4f30084b4.png">


Once you have these postgres and rust installed, you can follow these steps to get the CMS up and running:

Clone this repository to your local machine

    git clone https://github.com/TECHFUND/hiwrite

Navigate to the project directory and build the project

      cd hiwrite
      cargo build

Create a .env file in the project root and set the following environment variables:

    DATABASE_URL=postgres://[username]:[password]@localhost/rust_cms

Migrate the database

    cargo run -- migrate

Start the server

    cargo run -- server

The CMS should now be up and running at `http://localhost:8000`

If everything ran well you should see following prompt 

<img width="1131" alt="success" src="https://user-images.githubusercontent.com/4836107/207767002-3616231d-2c1b-4ea0-9df6-8d29b5b31f76.png">


## API

The CMS exposes a RESTful API that allows you to manage the content of your site. 

The API documentation can be found at http://localhost:8000/api/docs.


License

This project is licensed under the gplv3 License. See LICENSE for more details.
