'use strict';
const httpsRequest = require('https').request;

function setGitHubStatus(config, callback) {
  const request = httpsRequest({
    hostname: 'api.github.com',
    path: `/repos/${config.repo}/statuses/${config.sha}`,
    method: 'POST',
    headers: {
      'user-agent': 'node',
      authorization: 'token e3f5de775f8615eb2123c6eacec51c038dc0bd9b'
    }
  }, response => {
    response.on('error', err => {
      callback(502, {
        error: `An error occurred while receiving response from GitHub: ${JSON.stringify(err)}`
      });
    }).on('data', () => {
    }).on('end', () => {
      callback(200, {});
    });
  }).on('error', err => {
    callback(502, {
      error: `An error occurred while making request to GitHub: ${JSON.stringify(err)}`
    });
  });

  request.write(JSON.stringify({
    state: config.state,
    target_url: config.target_url,
    description: 'Status update posted from a cloud function.',
    context: 'spl-case-study'
  }));
  request.end();
}

/* TODO (OW)
function main(params) {
}
*/

/* TODO
exports.mainAWS = function(event, context, callback) {
};
*/

exports.mainGCP = function(req, res) {
  res.set('content-type', 'text/plain');
  setGitHubStatus(req.body, function(code, message) {
    res.status(code).json(message);
  });
};
