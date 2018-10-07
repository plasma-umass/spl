'use strict';

function base64tobinary(data, res) {
	res.status(200).send(Buffer.from(data.toString(), 'base64'));
};

exports.base64tobinary_GCF = function(req, res) {
	try {
    base64tobinary(req.rawBody, res);
	} catch(e) {
		res.status(400).send('Error happened: ' + e);
  }
};

