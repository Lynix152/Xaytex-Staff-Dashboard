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

````dockerfile
docker run -p 8080:8080 -p 3306:3306 --name xaytex-staff-dashboard -d xaytex/xaytex-staff-dashboard
````
 

## License

This project is licensed under the [MIT license](LICENSE).


## Project tree


````
Project: Xaytex-Staff-Dashboard
Xaytex-Staff-Dashboard/
    dashboard
        ressources
            images
                |--- logo.png
            views
                |--- index.html
            
        src
            data
                database
                    |--- db.rs
                |--- mod.rs
            domain
                |--- github.rs
                |--- routes.rs
            infastructure
                logging
                    |--- config.rs
                    |--- error.rs
                    |--- logger.rs
                    |--- middleware.rs
                    |--- mod.rs
                    |--- router.rs
                server
                    |--- mod.rs
                    |--- config.rs
                    |--- error.rs
                    |--- server.rs
                mod.rs
            |--- main.rs
            |--- mod.rs
            |--- contribuitor.rs
            
        test
            |--- main.rs
            |--- mod.rs   
            
    |--- Cargo.lock
    |--- Cargo.toml
    |--- config.yml
    |--- dashboard.env
    
|--- LICENSE
|--- README.md
````    