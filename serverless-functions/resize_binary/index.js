'use strict';
const gm = require('gm').subClass({imageMagick: true});

// from https://github.com/aheckmann/gm/issues/572
// better error messaging than gm.toBuffer()
function gmToBuffer (data) {
  return new Promise((resolve, reject) => {
    data.stream((err, stdout, stderr) => {
      if (err) { return reject(err) }
      const chunks = []
      stdout.on('data', (chunk) => { chunks.push(chunk) })
      // these are 'once' because they can and do fire multiple times for multiple errors,
      // but this is a promise so you'll have to deal with them one at a time
      stdout.once('end', () => { resolve(Buffer.concat(chunks)) })
      stderr.once('data', (data) => { reject(String(data)) })
    })
  })
}

function resize(data, params, res) {
  // 100 is default, could be anything
  const width = (params.width === undefined) ? "100" : params.width,
        height = (params.height === undefined) ? "100" : params.height;

  const buf = gm(Buffer.from(data)).resize(width, height)
  gmToBuffer(buf).then( function(buffer) {
      res.status(200).send(buffer);
  })
  .catch(function(err){
      res.status(500).send(err.toString());
  })
}

exports.resize_binary = function(req, res) {
	try {
    resize(req.rawBody, req.query, res);
	} catch(e) {
		res.status(400).send('Error happened: ' + e);
  }
};

