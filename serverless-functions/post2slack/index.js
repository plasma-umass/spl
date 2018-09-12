'use strict';
const https = require('https');

/**
 * Take a JSON request and POST to Slack; respond with success/fail.
 *
 * @param {!express:Request} req HTTP request context.
 * @param {!express:Response} res HTTP response context.
 */
exports.main = function(req, res) {
  const request = https.request({
    hostname: 'slack.com',
    path: '/api/chat.postMessage',
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      authorization: '<TOKEN HERE>'
    }
  }, function(response) {//One-time listener for the response event.
    var payload = [];

    response.on('error', function(e) {//Error making call to Slack API.
      res.status(500).json({
          error: `An error occured while attempting to post to Slack; ${e}.`
      });
    }).on('data', function(chunk) {//Receiving...
      payload.push(chunk);
    }).on('end', function() {//Done; response received in full.
      payload = JSON.parse(String(Buffer.concat(payload)));
      res.status(payload.ok ? 200 : 500).json({
          info: 'POST attempt complete; see included results from Slack.',
          slack: payload
      });
    });
  });

  request.write(req.body);
  request.end();
};
