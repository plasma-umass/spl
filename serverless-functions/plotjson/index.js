'use strict';

const Functions = require('./functions');

exports.plotjson = (req, res) => {
  res.send(Functions.plotjson(req.body, req.query));
};
