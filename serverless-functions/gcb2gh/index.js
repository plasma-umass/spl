'use strict';
const JWT = require('google-auth-library').JWT,
  httpsRequest = require('https').request,
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
      console.error('An error occurred while attempting to post build status to GitHub:', JSON.stringify(err));
      callback();
    }).on('end', () => {
      callback();
    });
  });

  request.write(JSON.stringify({
    state: config.state,
    target_url: config.url,
    description: 'Status update from Google Cloud Build',
    context: 'arjunguha-research-group/google-cloud-build'
  }));
  request.end();
}

exports.main = function(event, callback) {
  const data = JSON.parse(String(Buffer.from(event.data.data, 'base64'))),
    src = data.sourceProvenance.resolvedRepoSource;

  authClient.authorize().then(() => {
    authClient.request({
      url: `https://content-sourcerepo.googleapis.com/v1/projects/umass-plasma/repos/${src.repoName}`
    }).then(val => {
      const repoUrl = val.data.mirrorConfig.url,
        buildState = statusToStateMap[data.status];

      if(buildState) {
        setGitHubStatus({
          state: buildState,
          sha: src.commitSha,
          url: data.logUrl,
          repo: repoUrl.slice(repoUrl.indexOf('github.com/') + 11, repoUrl.indexOf('.git'))
        }, callback);
      } else {
        console.error('Unknown status:', JSON.stringify(data));
        callback();
      }
    }, err => {
      console.error('Google source repo request failure:', JSON.stringify(err));
      callback();
    });
  }, err => {
    console.error('Google API auth failure:', JSON.stringify(err));
    callback();
  });
};
