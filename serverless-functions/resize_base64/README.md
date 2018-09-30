# Resize base64 image 

## How to use
`curl -X POST 'https://us-central1-test-project-216622.cloudfunctions.net/resize_base64?width=100&height=100'  --data-binary @b64 > b64_resized`
Where `b64` is your input base64-encoded image and `b64_resized` is the resized image in base64, with dimensions `width` and `height`.
