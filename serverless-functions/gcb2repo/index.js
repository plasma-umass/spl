'use strict';
const JWT = require('google-auth-library').JWT,
  key = require('./key.json');

const statusToStateMap = {
  WORKING: 'pending',
  QUEUED: 'pending',
  TIMEOUT: 'failure',
  FAILURE: 'failure',
  SUCCESS: 'success'
},
authClient = new JWT({
  key: key.private_key,
  email: key.client_email,
  scopes: ['https://www.googleapis.com/auth/cloud-platform']
});

function getRepoInfo(data, callback) {
  const src = data.sourceProvenance.resolvedRepoSource;

  authClient.authorize().then(() => {
    authClient.request({
      url: `https://content-sourcerepo.googleapis.com/v1/projects/umass-plasma/repos/${src.repoName}`
    }).then(val => {
      const repoUrl = val.data.mirrorConfig.url,
        buildState = statusToStateMap[data.status];

      buildState ? callback(200, {
        state: buildState,
        sha: src.commitSha,
        target_url: data.logUrl,
        repo: repoUrl.slice(repoUrl.indexOf('github.com/') + 11, repoUrl.indexOf('.git'))
      }) : callback(500, {
        error: `Unknown status: ${JSON.stringify(data)}`
      });
    }, err => {
      callback(502, {
        error: `Google repo API request failure: ${JSON.stringify(err)}`
      });
    });
  }, err => {
    callback(502, {
      error: `Google auth API request failure: ${JSON.stringify(err)}`
    });
  });
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
  getRepoInfo(req.body, function(code, message) {
    res.status(code).json(message);
  });
};
