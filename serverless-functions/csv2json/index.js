'use strict';
const csv = require('csvjson');

function csv2json(data) {
  return csv.toArray(data.toString());
}

exports.mainAWS = function(event, context, callback) {
  try {
    callback(null, csv2json(JSON.parse(event.csv)));
  } catch(e) {
    callback(e);
  }
};

exports.mainGCP = function(req, res) {
  try {
    res.status(200).send(JSON.stringify(csv2json(req.body)));
  } catch(e) {
    res.status(400).send(e);
  }
};
