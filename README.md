# seminar-work-web-app
A blog platform written in Rust with the Rocket framework

## Requirements
You will need [Rust](https://www.rust-lang.org/tools/install) and Cargo,
the official Rust package manager, which should come with Rust.  

NOTE: This project uses the nightly and not the stable Rust version.  

You will also need a SQL database server, like for example [MariaDB](https://mariadb.org/).  
Finally, you will need [Git](https://git-scm.com/) to download this project (or you can click Code->Download Zip above).

## Setup
Clone this repository with git:  

```git clone https://github.com/vukanovics/seminar-work-web-app.git```  

Switch the working directory to the project:  

```cd seminar-work-web-app```  

Ensure Cargo is using the nightly version instead of stable:  

```rustup override set nightly```  

Install diesel_cli with Cargo:  

```cargo install diesel_cli```  

Create a ```.env``` file in the project root with the database info:  

```DATABASE_URL=mysql://<username>:<password>@localhost/<database_name>```  

Run all diesel migrations to setup the tables in the database:  

```diesel migration run```  

And finally build and run the project with cargo:  

```cargo run```  

## Features
- Account registration and login
- Posts are shown with the title and a description on the root page
- Posts can be created by all registered users
- Post content can contain markdown, rendered on the dedicated post page
- You can click a post title on the front page to go to the dedicated post page
