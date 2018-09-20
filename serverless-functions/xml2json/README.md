# xml2json

`xml2json` takes the POST data associated with the HTTP request as XML that is to be converted into JSON and returned. An example format using cURL is included below.

```bash
curl -X POST -H 'content-type: text/xml' -d '<raw XML here>' https://us-east1-umass-plasma.cloudfunctions.net/xml2json
```
