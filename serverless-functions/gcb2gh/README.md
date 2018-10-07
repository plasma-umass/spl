# Google Cloud Build to GitHub

This serverless-function has a pub-sub trigger. When a Google Cloud Build of the associated project emits an event, this function is triggered. The aforementioned event contains data regarding the build, which can be used to obtain the precise GitHub repository with an HTTP GET. Once the exact repository is known, the function will POST the build information to GitHub.
