"use strict";

const request = require('request');

const url = "https://api.census.gov/data/timeseries/asm/industry?get=NAICS_TTL,EMP,GEO_TTL&for=us:*&YEAR=2003,2004,2005,2006,2007,2008,2009,2010,2011,2012,2013,2014,2015,2016&NAICS=31-33"

function censusdata(callback) {
  request({
    url: url,
    json: true
  }, function (error, response, body) {
    if (!error && response.statusCode === 200) {
      const tupleData = body.slice(1).map(function(row) {
        return {
          "Jobs": row[1],
          "Year": row[3]
        };
      });

      return callback(JSON.stringify(tupleData), response.statusCode);
    } else {
      return callback(error, response.statusCode);
    }
  })
}

exports.main = function(req, res) {
  censusdata(function(output, status) {
    res.set("content-type", status === 200 ? "application/json" : "text/plain");
    res.status(status).send(output);
  });
};
