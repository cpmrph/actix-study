# actix-study

```bash
cargo run
```

## Sample

```bash
> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "CREATE_SESSION", "data": {"user_name": "hoge", "flag": true}}' 127.0.0.1:8080/session
{"result":"SUCCESS","data":{"user_name":"hoge","session_id":"1"}}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "CREATE_SESSION", "data": {"user_name": "", "flag": true}}' 127.0.0.1:8080/session
{"result":"FAILURE","data":{"error_code":"SESSION_CREATION_ERROR"}}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "CREATE_SESSION", "data": {"user_name": "John Doe", "flag": true}}' 127.0.0.1:8080/session
{"result":"FAILURE","data":{"error_code":"SESSION_CREATION_ERROR2"}}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "CREATE_SESSION", "data": {"flag": true}}' 127.0.0.1:8080/session
{"error_code":"ACTIX_WEB_ERROR"}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "JOIN_SESSION", "data": {"user_name": "hoge"}}' 127.0.0.1:8080/session
{"result":"SUCCESS","data":{"user_name":"hoge","session_id":"1","joined_at":"2023-10-05T12:34:56Z"}}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "JOIN_SESSION", "data": {"user_name": ""}}' 127.0.0.1:8080/session
{"result":"FAILURE","data":{"error_code":"SESSION_JOIN_ERROR","detail":"name required"}}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "JOIN_SESSION"}' 127.0.0.1:8080/session
{"error_code":"ACTIX_WEB_ERROR"}

> curl -X POST -H "Content-Type: application/json" -i -d '{"action": "JOHN_SESSION"}' 127.0.0.1:8080/session
{"error_code":"ACTIX_WEB_ERROR"}
```
