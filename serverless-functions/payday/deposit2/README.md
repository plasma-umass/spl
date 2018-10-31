## deposit2

Working example of the corresponding code found in _Figure 2_ of the paper. Motivates how serverless function should _not_ be composed.

#### Schema:

```json
{
  "to1": 00,
  "to2": 00,
  "amnt1": 00,
  "amnt2": 00,
  "tId1": "<transaction id here>",
  "tId2": "<transaction id here>"
}
```

#### Example:
```bash
$ gcloud functions call deposit2 --region us-east1 --data '{"to1": 564440, "to2": 562949, "amnt1": 10, "amnt2": 10, "tId1": "1", "tId2":"2"}'
executionId: uaivbaug9foz
result: 'true'
```
