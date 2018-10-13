'use strict';
const Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  const dsClient = new Datastore({ projectId: 'umass-plasma' }),
    dsClientTrans = dsClient.transaction();

  dsClientTrans.run(() => {
    const transId = dsClient.key(['Transaction', req.body.transId]);

    dsClient.get(transId, (err, trans) => {
      function postTrans(success, failure) {
        dsClient.insert({ key: transId, data: {} }, (err) => {
          err ? dsClientTrans.rollback(() => { res.send(failure); }) :
            dsClientTrans.commit(() => { res.send(success); });
        });
      }

      if(err || trans) {
        dsClientTrans.rollback(() => { res.send('Invalid transaction ID.'); });
      } else {
        if(req.body.type.trim().toLowerCase() === 'deposit') {
          const acctNum = dsClient.key(['Account', req.body.to]);

          dsClient.get(acctNum, (err, acct) => {
            if(err || !acct) {
              dsClientTrans.rollback(() => { res.send('Retrieve account failed.'); });
            } else {
              acct.Balance += req.body.amount;
              dsClient.update({ key: acctNum, data: acct }, (err) => {
                err ? dsClientTrans.rollback(() => { res.send('Update account failed.'); }) :
                  postTrans('Deposit complete.', 'Update account failed.');
              });
            }
          });
        } else if(req.body.type.trim().toLowerCase() === 'transfer') {
          const amnt = req.body.amount, acctNumFrom = dsClient.key(['Account', req.body.from]),
            acctNumTo = dsClient.key(['Account', req.body.to]);

          dsClient.get([acctNumFrom, acctNumTo], (err, accts) => {
            if(err || !accts[0] || !accts[1]) {
              dsClientTrans.rollback(() => { res.send('Retrieve accounts failed.'); });
            } else {
              if(accts[0].Balance >= amnt) {
                accts[0].Balance -= amnt; accts[1].Balance += amnt;
                dsClient.update([{ key: acctNumFrom, data: accts[0] },
                  { key: acctNumTo, data: accts[1] }], (err) => {
                  err ? dsClientTrans.rollback(() => { res.send('Update accounts failed.'); }) :
                    postTrans('Transfer complete.', 'Update accounts failed.');
                });
              } else {
                dsClientTrans.rollback(() => { res.send('Insufficient funds.'); });
              }
            }
          });
        } else {
          dsClientTrans.rollback(() => { res.send(`Unknown operation: ${req.body.type}.`); });
        }
      }
    });
  });
};
