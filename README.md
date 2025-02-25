# dpb-rs
A simple pastebin powered by ~~Django~~ rocket. This is a rust version of the original [dpb](https://github.com/cjc7373/dpb/tree/python). This project is to practice my rust skills. 

I don't want to use an external service, so I choose sqlite.

## Run with docker
`docker run -p 8080:8080 -v ./data:/app/data cjc7373/dpb:1.0`
