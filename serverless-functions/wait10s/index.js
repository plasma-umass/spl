"use strict";

function dumbSleep(millis) {
  const startMillis = (new Date()).getTime();
  const endMillis = startMillis + millis;

  while((new Date()).getTime() < endMillis) {}
}

exports.wait10s_GCF = function(req, res) {
  dumbSleep(10000);

  res.set("content-type", req.headers['content-type'] || "text/plain");
  res.status(200).send(req.body);
};




