# Resize binary image 

## How to use
`curl -X POST 'https://us-central1-test-project-216622.cloudfunctions.net/resize_binary?width=100&height=100'  --data-binary @img.jpg > resized.jpg`
Where `img.jpg` is your input image and `resized.jpg` is the resized image, with dimensions `width` and `height`.
