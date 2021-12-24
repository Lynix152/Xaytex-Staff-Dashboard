# Xaytex-Staff-Dashboard


## Xaytex Staff Dashboard

If you're a Xatex Member, you can control everything for your account. 

If you have a project, you can get from our project lead [Koni](https://github.com/vKxni) breafings.

You can chat with him, or with other Developers, you can control your work, 
also share code with him



## Run Project

-`git clone https://github.com/Xaytex/Xaytex-Staff-Dashboard.git`

-`cd Xaytex-Staff-Dashboard`

-`cargo run`


 ## Run Project with Docker

 -`docker run -p 8080:8080 -p 3306:3306 --name xaytex-staff-dashboard -d xaytex/xaytex-staff-dashboard`
 

## License

This project is licensed under the [MIT license](LICENSE).


## Project tree


````
[//]: # ( Path: src/main.rs

|--- README.md

|--- LICENSE

|--- Dashboard

        |--- Cargo.toml

        |--- Cargo.lock

        |--- src

            |--- main.rs

            |--- routes.rs

            |--- views

                |--- index.html

            |--- data

                |--- db.sqlite

                |--- schema.sql

            |--- images

                |--- logo.png
