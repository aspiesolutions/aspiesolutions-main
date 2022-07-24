# migrate
this tool simply runs the equivelent of 'sea-orm-cli' migrate up
this approach was chosen because "cargo install sea-orm-cli" would error in github actions if the tool was already installed
this approach allows us to simply run cargo run --bin migrate and github actions has caching to reduce build times