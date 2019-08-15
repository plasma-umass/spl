# post2slack

`post2slack` takes the POST data associated with the HTTP request as information for posting a message in the PLASMA Slack instance. The POST body parameters must include `channel` and `text`, both of which are strings. Additional optional parameters can be found on the [Slack API docs](https://api.slack.com/methods/chat.postMessage). An example format using cURL is included below.

```bash
curl -X POST -H 'content-type: application/json' -d '{"channel":"<channel name here>","text":"<message text here>"}' https://us-east1-umass-plasma.cloudfunctions.net/post2slack
```

**ATTN:** The [auth token](https://github.com/plasma-umass/spl/blob/master/serverless-functions/post2slack/index.js) has been disabled by Slack since this repo was made public. A new token must be generated and the app redeployed in order for it to function as intended.
