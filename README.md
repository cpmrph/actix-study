# actix-study

```bash
cargo run
```

## Sample

```bash
# Create room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"5c2d0243-128b-42ab-8427-2a58ed4bcf8f"}' 127.0.0.1:8080/room

# Subscrive room events
curl -X POST -H "Content-Type: application/json" -d '{"version":2}' 127.0.0.1:8080/room/2db318cb-35f5-4757-8ded-715053abcd5a/subscribe

# Add user to room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"facd39d3-733b-42e4-9200-17f2dd9e68f1"}' 127.0.0.1:8080/room/2db318cb-35f5-4757-8ded-715053abcd5a/join

# Get room
curl 127.0.0.1:8080/room/2db318cb-35f5-4757-8ded-715053abcd5a

# Remove user from room
curl -X POST -H "Content-Type: application/json" -d '{"user_id":"facd39d3-733b-42e4-9200-17f2dd9e68f1"}' 127.0.0.1:8080/room/2db318cb-35f5-4757-8ded-715053abcd5a/leave
```
