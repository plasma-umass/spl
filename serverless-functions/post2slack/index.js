'use strict';
const https = require('https');

function post2slack() {
  //TODO
}

function main(params) {
  //TODO
}

exports.mainAWS = function(event, context, callback) {
  //TODO
};

exports.mainGCP = function(req, res) {
  const request = https.request({
    hostname: 'slack.com',
    path: '/api/chat.postMessage',
    method: 'POST',
    headers: {
      'content-type': 'application/json;charset=utf-8',
      authorization: 'Bearer xoxp-2521834784-429986587315-435464683666-81eea6abe2ecc26b3469622741021ea6'
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

  request.write(JSON.stringify(req.body));
  request.end();
};
