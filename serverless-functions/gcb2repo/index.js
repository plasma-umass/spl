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

exports.main = function(event, callback) {
  const data = JSON.parse(String(Buffer.from(event.data.data, 'base64'))),
    src = data.sourceProvenance.resolvedRepoSource;

  authClient.authorize().then(() => {
    authClient.request({
      url: `https://content-sourcerepo.googleapis.com/v1/projects/umass-plasma/repos/${src.repoName}`
    }).then(val => {
      const repoUrl = val.data.mirrorConfig.url,
        buildState = statusToStateMap[data.status];

      buildState ? callback(null, {
        state: buildState,
        sha: src.commitSha,
        target_url: data.logUrl,
        repo: repoUrl.slice(repoUrl.indexOf('github.com/') + 11, repoUrl.indexOf('.git'))
      }) : callback(`Unknown status: ${JSON.stringify(data)}`);
    }, err => {
      callback(`Google repo API request failure: ${JSON.stringify(err)}`);
    });
  }, err => {
    callback(`Google auth API request failure: ${JSON.stringify(err)}`);
  });
};
