# actix-study

```bash
cargo run
```

## Sample

```bash
# Create room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"5c2d0243-128b-42ab-8427-2a58ed4bcf8f"}' 127.0.0.1:8080/room

# Add user to room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"facd39d3-733b-42e4-9200-17f2dd9e68f1"}' 127.0.0.1:8080/room/b5e943f3-0bb5-4203-92f5-9a219e76be45/join

# Get room
curl 127.0.0.1:8080/room/b5e943f3-0bb5-4203-92f5-9a219e76be45

# Remove user from room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"facd39d3-733b-42e4-9200-17f2dd9e68f1"}' 127.0.0.1:8080/room/b5e943f3-0bb5-4203-92f5-9a219e76be45/leave
```
