# Primative Bank Cloud Function

This cloud function is the working manifestation of the that which is detailed in the "Overview" section of the paper, and shown in _Figure 1b_. As described, it addresses the three major concerns with naive development of serverless functions, such that readers/programmers are aware of this alternative paradigm.

1. **Persistence:** This is handled via integration with Google Cloud Datastore; a NoSQL document DB. There are two kinds of objects stored:
    1. Account:
        - Key:
            Integer (auto-generated)
        - Value:
          ```json
          {
            "Name": "name",
            "Balance": 00
          }
          ```
    2. Transaction:
        - Key:
            String (user-generated)
        - Value:
            ```json
            {}
            ```
2. **Concurrency:** Dealt with by way of transactions. These are natively support by the Cloud Datastore APIs and force the containing operations to behave atomically. Furthermore, within a transaction the data that is referenced cannot be read or modified externally until completion. In locking the pertinent resources for the duration of a transaction, the system is safe from concurrent execution.

3. **Idempotents:** To be sure an invocation is only committed once, the client must pass a transaction identifier. Once the transaction begins, a lookup is performed to examine if the corresponding identifier exists in the datastore. Only if it is _not_ found does the transaction proceed. If/when the subsequent transaction operations succeed, the identifier is written to the persistence layer, thereby completing the transaction.

## Caller Interface

Two operations are supported. Their data schemas are as follows:
1. Deposit:
    ```json
    {
      "type": "deposit",
      "to": 00,
      "amount": 00,
      "transId": "<transaction id here>"
    }
    ```
2. Transfer:
    ```json
    {
      "type": "transfer",
      "from": 00,
      "to": 00,
      "amount": 00,
      "transId": "<transaction id here>"
    }
    ```
#### Examples using `gcloud`:
```bash
$ gcloud functions call bank --region us-east1 --data '{"type": "deposit", "to": 563944, "amount": 10, "transId": "5"}'
executionId: zsmwax1lsq0w
result: Deposit complete.
$ gcloud functions call bank --region us-east1 --data '{"type": "transfer", "to": 563944, "from": 562949, "amount": 20, "transId": "6"}'
executionId: zsmwax5apzsh
result: Transfer complete.
$ gcloud functions call bank --region us-east1 --data '{"type": "transfer", "to": 563944, "from": 562949, "amount": 20, "transId": "6"}'
executionId: zsmwtly5v3a5
result: Invalid transaction ID.
```
