'use strict';
const Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  const dsClient = new Datastore({
    projectId: 'umass-plasma'
  }),
    dsClientTrans = dsClient.transaction();

  dsClientTrans.run(function() {
    const transId = dsClient.key(['Transaction', req.body.transId]);

    dsClient.get(transId, function(err, trans) {
      if(err || trans) {
        dsClientTrans.rollback(function() {
          res.send('Invalid transaction ID.');
        });
      } else {
        if(req.body.type === 'deposit') {
          const acctNum = dsClient.key(['Account', req.body.to]);

          dsClient.get(acctNum, function(err, acct) {
            if(err || !acct) {
              dsClientTrans.rollback(function() {
                res.send('Failed to retrieve the account.');
              });
            } else {
              acct.Balance += req.body.amount;
              dsClient.update({
                key: acctNum,
                data: acct
              }, function(err) {
                if(err) {
                  dsClientTrans.rollback(function() {
                    res.send('Failed to update the account.');
                  });
                } else {
                  dsClient.insert({
                    key: transId,
                    data: {}
                  }, function(err) {
                    if(err) {
                      dsClientTrans.rollback(function() {
                        res.send('Failed to update the account.');
                      });
                    } else {
                      dsClientTrans.commit(function() {
                        res.send('Deposit complete.');
                      });
                    }
                  });
                }
              });
            }
          });
        } else if(req.body.type === 'transfer') {
          const amnt = req.body.amount,
            acctNumFrom = dsClient.key(['Account', req.body.from]),
            acctNumTo = dsClient.key(['Account', req.body.to]);

          dsClient.get([acctNumFrom, acctNumTo], function(err, accts) {
            if(err || !accts[0] || !accts[1]) {
              dsClientTrans.rollback(function() {
                res.send('Failed to retrieve the accounts.');
              });
            } else {
              if(accts[0].Balance >= amnt) {
                accts[0].Balance -= amnt;
                accts[1].Balance += amnt;
                dsClient.update([{
                  key: acctNumFrom,
                  data: accts[0]
                }, {
                  key: acctNumTo,
                  data: accts[1]
                }], function(err) {
                  if(err) {
                    dsClientTrans.rollback(function() {
                      res.send('Failed to update the accounts.');
                    });
                  } else {
                    dsClient.insert({
                      key: transId,
                      data: {}
                    }, function(err) {
                      if(err) {
                        dsClientTrans.rollback(function() {
                          res.send('Failed to update the accounts.');
                        });
                      } else {
                        dsClientTrans.commit(function() {
                          res.send('Transfer complete.');
                        });
                      }
                    });
                  }
                });
              } else {
                dsClientTrans.rollback(function() {
                  res.send('Insufficient funds.');
                });
              }
            }
          });
        } else {
          dsClientTrans.rollback(function() {
            res.send(`Unknown operation: ${req.body.type}.`);
          });
        }
      }
    });
  });
};
