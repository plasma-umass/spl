"use strict";
const vega = require("vega");

function plotjson(jsonBody, getQuery) {
  const xName = (getQuery && getQuery.xname) || "x",
        yName = (getQuery && getQuery.yname) || "y",
        renamedData = [];

  // Validate that JSON is an array
  if(!Array.isArray(jsonBody)) {
    return Promise.reject({
      message: `JSON input is not an array, instead we received: ${JSON.stringify(jsonBody)}`,
      status: 400
    });
  }

  for(const pair of jsonBody) {
    if((xName in pair) && (yName in pair)) {
      renamedData.push({x: pair[xName], y: pair[yName]});
    } else {
      return Promise.reject({
        message: `JSON input pair ${JSON.stringify(pair)} does not contain both required keys "${xName}" and "${yName}".`,
        status: 400
      });
    }
  }

  const plot_spec = {
    "$schema": "https://vega.github.io/schema/vega/v4.json",
    "width": 400,
    "height": 200,
    "padding": 5,
    "config": {
      "background": "white"
    },
    "data": [{
      "name": "table",
      "values": renamedData
    }],
    "scales": [{
      "name": "xscale",
      "type": "point",
      "range": "width",
      "domain": {
        "data": "table",
        "field": "x"
      }
    }, {
      "name": "yscale",
      "type": "linear",
      "range": "height",
      "nice": true,
      "zero": true,
      "domain": {
        "data": "table",
        "field": "y"
      }
    }],
    "axes": [{
      "orient": "bottom",
      "scale": "xscale",
      "title": xName,
      "labels" : false,
      "ticks" : false
    }, {
      "orient": "left",
       "scale": "yscale",
       "title": yName,
    }],
    "marks": [{
      "type": "line",
      "from": {
        "data": "table"
      },
      "encode": {
        "enter": {
          "x": {
            "scale": "xscale",
            "field": "x",
          },
          "y": {
            "scale": "yscale",
            "field": "y"
          },
          "strokeWidth": {
            "value": 1
          }
        }
      }
    }]
  };

  const view = new vega.View(vega.parse(plot_spec))
    .logLevel(vega.Warn) // set view logging level
    .renderer("svg") // set render type (defaults to "canvas")
    .run(); // update and render the view

  return view.toImageURL("png", 2).then(function(url) {
    // Remove the first occurrence of the data type header thingy
    return Buffer.from(url.replace("data:image/png;base64,", ""), "base64");
  }, function(err) {
    return Promise.reject({
      message: err,
      status: 500
    });
  });
}

exports.mainAWS = function(event, context, callback) {
  plotjson(event.data, event.names).then(function(out) {
    callback(null, out);
  }, function(err) {
    callback(JSON.stringify(err));
  });
};

exports.mainGCP = function(req, res) {
  plotjson(req.body, req.query).then(function(out) {
    res.set("content-type", "image/png");
    res.status(200).send(out);
  }, function(err) {
    res.set("content-type", "text/plain");
    res.status(err.status).send(err.message);
  });
};
