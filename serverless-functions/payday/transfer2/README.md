## transfer2

Working example of the corresponding code found in _Figure 2_ of the paper. Motivates how serverless function should _not_ be composed.

#### Schema:

```json
{
  "from": 00,
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
$ gcloud functions call transfer2 --region us-east1 --data '{"from": 5644406560391168, "to1": 5639445604728832, "to2": 5629499534213120, "amnt1": 10, "amnt2": 10, "tId1": "11", "tId2":"12"}'
executionId: c4cxylkz6n7i
result: Transfers complete.
```
