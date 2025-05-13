# actix-study

```bash
cargo run
```

## Sample

```bash
# Create room
curl -X POST 127.0.0.1:8080/rooms

# Add user to room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"1ab6bed8-13da-4bb2-a9f2-e9db6ffabba3"}' 127.0.0.1:8080/rooms/4d33bdb3-f4c6-4fc6-aff3-79ee0b177503/join

# Remove user from room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"1ab6bed8-13da-4bb2-a9f2-e9db6ffabba3"}' 127.0.0.1:8080/rooms/4d33bdb3-f4c6-4fc6-aff3-79ee0b177503/leave
```
