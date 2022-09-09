# Default Backend
This is a very simple rust binary which presents a json response as a 404

## Environment variables
The listening IP, Port and the reponse text can all be customised via environment variables

| Name | Type | Default |
|---|---|---|
| IP | String | 0.0.0.0 |
| PORT | String | 8080 |
| RESP_TEXT | String | {"error": "not found"} |