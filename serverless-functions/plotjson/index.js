'use strict';

const exec = require('child_process').exec;


/**
 *
 * @param {Object} req A Javascript object of the JSON body, or null if there is no parsable JSON body
 * @param {Object} res A Javascript object of the GET query.
 */
function plotjson(jsonBody, getQuery, callback) {
    exec('ls /usr/bin', {stdio: 'ignore'}, (err, stdout) => {
        callback(stdout);
    });
    // callback(`Hello ${jsonBody.name || 'World'}!`);
}

exports.plotjson_GCF = (req, res) => {
  plotjson(req.body, req.query, output => {
    res.send(output)
  });
};
