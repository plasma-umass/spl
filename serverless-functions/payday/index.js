'use strict';
const Datastore = require('@google-cloud/datastore');

exports.main = function(req, res) {
  const datastore = new Datastore({
    projectId: 'umass-plasma'
  });

  const key = datastore.key(['Account', 'Alice']);
  datastore.get(key, function(err, entity) {
    console.log(entity);
    res.sendStatus(err ? 500 : 200);
  });
};
