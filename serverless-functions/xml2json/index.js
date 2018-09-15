'use strict';
const parse = require('xml2json-light');

function xml2json(rawXML) {
  return parse.xml2json(`${rawXML}`);
}

function main(params) {
  //TODO
}

exports.mainAWS = function(event, context, callback) {
  //TODO
};

exports.mainGCP = function(req, res) {
  try {
    res.status(200).json(xml2json(req.body));
  } catch(e) {
    res.status(400).json({
      error: `Please be sure to include valid XML in the request payload; ${e}.`
    });
  }
};
