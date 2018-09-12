'use strict';
const parse = require('xml2json-light');

/**
 * Takes XML from the request body and responds with the equivalent JSON.
 *
 * @param {!express:Request} req HTTP request context.
 * @param {!express:Response} res HTTP response context.
 */
exports.main = function(req, res) {
  try {
    res.status(200).json(parse.xml2json(`${req.body}`));
  } catch(e) {
    res.status(400).json({
      error: `Please be sure to include valid XML in the request payload; ${e}.`
    });
  }
};
