# Image to Base64 encoding serverless function

## Send image and recieve base64 encoding
`curl -X POST https://us-central1-test-project-216622.cloudfunctions.net/img2base64_GCF --data-binary @img.jpg > out`  

Where `img.jpg` is your image, and `out` is the base 64 encoding.  

To test the base 64 encoding, run:  
`base64 -D out > test.jpg`  
where `out` is the same base 64 encoding from before and `test.jpg` is the image. 
