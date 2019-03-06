# Feeder

A self-hosted Rust feed analytics platform designed along the lines of [Feedburner](https://feedburner.google.com) and [FeedPress](https://feed.press).

## Development

### Docker
```
docker-compose build
docker-compose up api

# new tab
docker-compose exec api bash
diesel migration generate create_posts_table
```


### Structure
The repository is split into two parts which is heavily based on [https://github.com/ghotiphud/rust-web-starter](https://github.com/ghotiphud/rust-web-starter).

#### api
A Rust actix-web server that uses diesel. 

#### web
A static site.
