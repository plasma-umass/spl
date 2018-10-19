'use strict';
let Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  let dsClient = new Datastore({ projectId: 'umass-plasma' });
  let dsClientTrans = dsClient.transaction();

  dsClientTrans.run(function() {
    let transId = dsClient.key(['Transaction', req.body.transId]);

    dsClientTrans.get(transId, function(err, trans) {
      if(err || trans) {
        dsClientTrans.rollback(function() { res.json('Invalid transaction ID.'); });
      } else {
        if(req.body.type === 'deposit') {
          let acctNum = dsClient.key(['Account', req.body.to]);

          dsClientTrans.get(acctNum, function(err, acct) {
            acct.Balance += req.body.amount;
            dsClientTrans.save({ key: acctNum, data: acct });
            dsClientTrans.save({ key: transId, data: {} });
            dsClientTrans.commit(function() { res.json('Deposit complete.'); });
          });
        } else if(req.body.type === 'transfer') {
          let amnt = req.body.amount;
          let acctNumFrom = dsClient.key(['Account', req.body.from]);
          let acctNumTo = dsClient.key(['Account', req.body.to]);

          dsClientTrans.get([acctNumFrom, acctNumTo], function(err, accts) {
            if(accts[0].Balance >= amnt) {
              accts[0].Balance -= amnt;
              accts[1].Balance += amnt;
              dsClientTrans.save([{ key: acctNumFrom, data: accts[0] },
                { key: acctNumTo, data: accts[1] }]);
              dsClientTrans.save({ key: transId, data: {} });
              dsClientTrans.commit(function() { res.json('Transfer complete.'); });
            } else {
              dsClientTrans.rollback(function() { res.json('Insufficient funds.'); });
            }
          });
        } else {
          dsClientTrans.rollback(function() {
            res.json('Unknown operation: ' + req.body.type);
          });
        }
      }
    });
  });
};
