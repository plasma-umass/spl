'use strict';
let request = require('request-promise-native');

exports.deposit2 = function(req, res) {
  let { to1, to2, amnt1, amnt2, tId1, tId2 } = req.body;
  request.post({ url:'https://us-east1-umass-plasma.cloudfunctions.net/bank',
    json: { type: 'deposit', to: to1, amount: amnt1, transId: tId1 }})
  .then(function() {
    request.post({ url:'https://us-east1-umass-plasma.cloudfunctions.net/bank',
      json: { type: 'deposit', to: to2, amount: amnt2, transId: tId2 }})
    .then(function() { res.send(true); });
  });
};
