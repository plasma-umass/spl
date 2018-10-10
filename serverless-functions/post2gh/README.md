# post2gh

`post2gh` takes the POST data associated with the HTTP request as information for posting a status update to GitHub. The POST body parameters must include the `sha`, `state`, and `target_url`, as described [here](https://developer.github.com/v3/repos/statuses/#create-a-status). In the case of `sha`, the cloud function will append the hash provided to it in the body of the request as a path parameter in the POST to GitHub (as described in the documentation). Similarly, a final `repo` parameter in the format of `<owner>/<repo>` must be provided in the payload; this combines the two sequential path parameters required by GitHUb. An example format using cURL is included below.

```bash
curl -X POST -H 'content-type: application/json' -d '{"state":"<state of status here>","target_url":"<target url here>","sha":"<commit sha here>","repo":"<owner/repo here>"}' https://us-east1-umass-plasma.cloudfunctions.net/post2gh
```
