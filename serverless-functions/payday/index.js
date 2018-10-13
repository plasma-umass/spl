'use strict';
const Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  const datastoreClient = new Datastore({
    projectId: 'umass-plasma'
  }),
    datastoreClientTransaction = datastoreClient.transaction();

  datastoreClientTransaction.run(function() {
    const transactionId = datastoreClient.key(['Transaction', req.body.transactionId]);

    datastoreClient.get(transactionId, function(err, transaction) {
      if(err || transaction) {
        datastoreClientTransaction.rollback(function() {
          res.send('Invalid \'transactionId\'.');
        });
      } else {
        if(req.body.type === 'deposit') {
          const accountNumber = datastoreClient.key(['Account', req.body.to]);

          datastoreClient.get(accountNumber, function(err, account) {
            if(err || !account) {
              datastoreClientTransaction.rollback(function() {
                res.send('Failed to retrieve the account.');
              });
            } else {
              account.Balance += req.body.amount;
              datastoreClient.update({
                key: accountNumber,
                data: account
              }, function(err) {
                if(err) {
                  datastoreClientTransaction.rollback(function() {
                    res.send('Failed to update the account.');
                  });
                } else {
                  datastoreClient.insert({
                    key: transactionId,
                    data: {}
                  }, function(err) {
                    if(err) {
                      datastoreClientTransaction.rollback(function() {
                        res.send('Failed to update the account.');
                      });
                    } else {
                      datastoreClientTransaction.commit(function() {
                        res.send('Deposit complete.');
                      });
                    }
                  });
                }
              });
            }
          });
        } else if(req.body.type === 'transfer') {
          const amount = req.body.amount,
            accountNumberFrom = datastoreClient.key(['Account', req.body.from]),
            accountNumberTo = datastoreClient.key(['Account', req.body.to]);

          datastoreClient.get([accountNumberFrom, accountNumberTo], function(err, accounts) {
            if(err || !accounts[0] || !accounts[1]) {
              datastoreClientTransaction.rollback(function() {
                res.send('Failed to retrieve the accounts.');
              });
            } else {
              if(accounts[0].Balance >= amount) {
                accounts[0].Balance -= amount;
                accounts[1].Balance += amount;
                datastoreClient.update([{
                  key: accountNumberFrom,
                  data: accounts[0]
                }, {
                  key: accountNumberTo,
                  data: accounts[1]
                }], function(err) {
                  if(err) {
                    datastoreClientTransaction.rollback(function() {
                      res.send('Failed to update the accounts.');
                    });
                  } else {
                    datastoreClient.insert({
                      key: transactionId,
                      data: {}
                    }, function(err) {
                      if(err) {
                        datastoreClientTransaction.rollback(function() {
                          res.send('Failed to update the accounts.');
                        });
                      } else {
                        datastoreClientTransaction.commit(function() {
                          res.send('Transfer complete.');
                        });
                      }
                    });
                  }
                });
              } else {
                datastoreClientTransaction.rollback(function() {
                  res.send('Insufficient funds.');
                });
              }
            }
          });
        } else {
          datastoreClientTransaction.rollback(function() {
            res.send(`Unknown operation: ${req.body.type}.`);
          });
        }
      }
    });
  });
};
