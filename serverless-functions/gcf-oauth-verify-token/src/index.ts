import * as auth from 'google-auth-library';
import * as express from 'express';
import * as cors from 'cors';
import * as bodyParser from 'body-parser';

// TODO(arjun): Really? Baked in?
const clientId = '692270598994-1dp1lfi6s0dqg7h18f0h1ibrne4gi1bu.apps.googleusercontent.com';

const client = new auth.OAuth2Client(clientId);
export const main = express();

main.use(cors({ origin: '*' }));

main.post('/', bodyParser.json(), function(req, res) {
  client.verifyIdToken({
    idToken: req.body.token,
    audience: clientId
  }).then(ticket => {

    if (!ticket) {
      res.status(200).send(JSON.stringify({ 
        kind: 'error', 
        message: 'no ticket returned'
      }));
      return;
    }
    const payload = ticket.getPayload();
    if (!payload) {
      res.status(200).send(JSON.stringify({ 
        kind: 'error', 
        message: 'no payload returned'
      }));
      return;
    }
    const email = payload.email;
    if (email === undefined) {
      res.status(200).send(JSON.stringify({ 
        kind: 'error', 
        message: 'no email in payload'
      }));
      return;
    }
    res.status(200).send(JSON.stringify({ 
      kind: 'ok',
      email: email
    }));
    return;
  }).catch(reason => {
    res.status(200).send(JSON.stringify({
      kind: 'error',
      message: 'Exception raised while processing request',
      reason: String(reason)
    }));
  });
});