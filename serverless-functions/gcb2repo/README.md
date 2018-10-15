# Google Cloud Build to GitHub Repo

For the purposes of the LLSPL example, this serverless-function has a HTTP (instead of pub-sub) trigger. This is because pub-sub functions may only be invoked via `gcloud`, if not by the specified topic.

The schema for the request payload is:
```json
{
  "status": "status",
  "sourceProvenance": {
    "resolvedRepoSource": {
      "projectId": "projectId",
      "repoName": "repoName",
      "commitSha": "commitSha"
    }
  },
  "logUrl": "logUrl"
}
```

An example invocation is:
```bash
curl -X POST -H 'content-type: application/json' -d '{"status": "QUEUED","sourceProvenance": {"resolvedRepoSource": {"projectId": "umass-plasma","repoName": "github-plasma-umass-spl","commitSha": "59f0a2e2abe5f83df8d45fc6920975932f5f0264"}},"logUrl": "https://console.cloud.google.com/gcr/builds/fd7865e4-1725-425d-bcd8-d70e867f9d18?project=494493709319"
}' https://us-east1-umass-plasma.cloudfunctions.net/gcb2repo
```
