'use strict';
const csv = require('csvjson')

exports.mainAWS = function(event, context, callback) {
	//TODO
};

function csv2json(data) {
	return csv.toObject(data.toString());
}

exports.csv2json_GCF = function(req, res) {
	try {
		res.status(200).send(JSON.stringify(csv2json(req.body)));
	} catch(e) {
		res.status(400).send('input must be csv as string: ' + e + '\n' + req.body);
  }
};

