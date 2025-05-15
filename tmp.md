# tmp

actix-web でレスポンスの json を以下の形式で返却したいです。
どのような型を定義すれば良いでしょうか？
Result 型、 serde/serde_json、thiserrorあたりを使用して実装して

```json
// セッション作成リクエスト成功時
{
  "result: "SUCCESS",
  "data": {
    "user_name": "John Doe"
  }
}
```

```json
// セッション作成リクエスト失敗時
{
  "result: "FAILURE",
  "data": {
    "error_code": "SESSION_ERROR"
  }
}
```

```json
// セッション参加リクエスト成功時
{
  "result: "SUCCESS",
  "data": {
    "session_id": "123",
    "user_name": "John Doe"
  }
}
```

```json
// セッション参加リクエスト失敗時
{
  "result: "FAILURE",
  "data": {
    "error_code": "JOIN_ERROR",
    "detail": "timeout"
  }
}
```
