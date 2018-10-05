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
      console.log('Response received.');
      const repoUrl = val.data.mirrorConfig.url,
        buildState = statusToStateMap[data.status];

      if(buildState) {
        callback(null, 'SUCCESS');
      } else {
        callback(`Unknown status: ${JSON.stringify(data)}`);
      }
    }, err => {
      console.error('Google repo API request failure:', JSON.stringify(err));
      callback();
    });
  }, err => {
    console.error('Google auth API request failure:', JSON.stringify(err));
    callback();
  });
};
