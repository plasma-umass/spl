'use strict';

function img2base64(data, res) {
	res.status(200).send(Buffer.from(data).toString('base64'));
	//res.status(200).send(data);
};

exports.img2base64_GCF = function(req, res) {
	try {
    img2base64(req.rawBody, res);
	} catch(e) {
		res.status(400).send('Error happened: ' + e);
  }
};

