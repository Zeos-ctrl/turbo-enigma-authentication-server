# turbo-enigma-authentication-server

This code runs locally using cargo run or via docker, the only thing that 
needs to be changed between the two is the database url. Its a code demo for
my module and is now not being added to as much after the units officially ended.
To run with Docker change the database urls in the code and use:
```
sudo docker build -t server .

sudo docker compose up
```

### TO-DO LIST

I'm not actively committing to this project again, but these things would be nice:

- [x] Add better comments
- [x] Get rid of the majority of unwraps
- [x] Database Migration Scripts
- [ ] TLS
