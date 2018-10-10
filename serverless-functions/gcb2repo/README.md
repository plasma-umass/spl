# Google Cloud Build to GitHub Repo

This serverless-function has a pub-sub trigger. When a Google Cloud Build of the associated project emits an event, this function is triggered. Addtionally this function can be invoked by `gcloud` with a mock event (to serve the case study for SPL). The aforementioned event contains data regarding the build, which can be used to obtain the precise GitHub repository with an HTTP GET.
