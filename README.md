# turbo-enigma-authentication-server

At the moment the database migrations arent done manually
so the docker container for mysql needs to be set up by itself.
The table commands to set it up are in ./migrations/20222010_up.sql

To use the application run:
```
sudo docker build -t server .

sudo docker compose up
```
### TO DO LIST

- [x] Add better comments
- [x] Get rid of the majority of unwraps
- [ ] Database Migration Scripts
- [ ] TLS
