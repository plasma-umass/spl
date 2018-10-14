'use strict';
let Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  let dsClient = new Datastore({ projectId: 'umass-plasma' });
  let dsClientTrans = dsClient.transaction();

  dsClientTrans.run(function() {
    let transId = dsClient.key(['Transaction', req.body.transId]);

    dsClient.get(transId, function(err, trans) {
      if(err || trans) {
        dsClientTrans.rollback(function() { res.send('Invalid transaction ID.'); });
      } else {
        if(req.body.type === 'deposit') {
          let acctNum = dsClient.key(['Account', req.body.to]);

          dsClient.get(acctNum, function(err, acct) {
            if(err || !acct) {
              dsClientTrans.rollback(function() { res.send('Retrieve account failed.'); });
            } else {
              acct.Balance += req.body.amount;
              dsClient.update({ key: acctNum, data: acct }, function() {
                dsClient.insert({ key: transId, data: {} }, function() {
                  dsClientTrans.commit(function() { res.send('Deposit complete.'); });
                });
              });
            }
          });
        } else if(req.body.type === 'transfer') {
          let amnt = req.body.amount;
          let acctNumFrom = dsClient.key(['Account', req.body.from]);
          let acctNumTo = dsClient.key(['Account', req.body.to]);

          dsClient.get([acctNumFrom, acctNumTo], function(err, accts) {
            if(err || !accts[0] || !accts[1]) {
              dsClientTrans.rollback(function() { res.send('Retrieve accounts failed.'); });
            } else {
              if(accts[0].Balance >= amnt) {
                accts[0].Balance -= amnt; accts[1].Balance += amnt;
                dsClient.update([{ key: acctNumFrom, data: accts[0] },
                  { key: acctNumTo, data: accts[1] }], function() {
                  dsClient.insert({ key: transId, data: {} }, function() {
                    dsClientTrans.commit(function() { res.send('Transfer complete.'); });
                  });
                });
              } else {
                dsClientTrans.rollback(function() { res.send('Insufficient funds.'); });
              }
            }
          });
        } else {
          dsClientTrans.rollback(function() { res.send(`Unknown operation: ${req.body.type}.`); });
        }
      }
    });
  });
};
