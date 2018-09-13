"use strict";

const vega = require("vega");


function plotjson(jsonBody, getQuery, callback) {
  const xName = getQuery.xname,
        yName = getQuery.yname;

  // Validate that JSON is an array
  if(!Array.isArray(jsonBody)) {
    callback("JSON input is not an array", 400);
    return;
  }

  // Remap the JSON input by renaming xName => "x", yName => "y"
  var mapFailed = false;
  const renamedData = jsonBody.map(function(pair) {
    if((xName in pair) && (yName in pair)) {
      const renamedPair = {x : pair[xName], y : pair[yName]};
      return renamedPair
    } else {
      callback(`JSON input pair ${JSON.stringify(pair)} does not contain both required keys "${xName}" and "${yName}".`, 400);
      mapFailed = true;
    }
  });

  if(mapFailed) {
    return;
  }

  const plot_spec = {
    "$schema": "https://vega.github.io/schema/vega/v4.json",
    "width": 400,
    "height": 200,
    "padding": 5,
    "config": {
      "background": "white"
    },
    "data": [
      {
        "name": "table",
        "values": renamedData
      }
    ],
    "scales": [
      {
        "name": "xscale",
        "type": "point",
        "range": "width",
        "domain": {
          "data": "table",
          "field": "x"
        }
      },
      {
        "name": "yscale",
        "type": "linear",
        "range": "height",
        "nice": true,
        "zero": true,
        "domain": {
          "data": "table",
          "field": "y"
        }
      }
    ],
    "axes": [
      {
        "orient": "bottom",
        "scale": "xscale",
        "title": xName
      },
      {
        "orient": "left",
        "scale": "yscale",
        "title": yName
      }
    ],
    "marks": [
      {
        "type": "line",
        "from": {
          "data": "table"
        },
        "encode": {
          "enter": {
            "x": {
              "scale": "xscale",
              "field": "x"
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
      }
    ]
  };

  const view = new vega.View(vega.parse(plot_spec))
    .logLevel(vega.Warn) // set view logging level
    .renderer("svg") // set render type (defaults to "canvas")
    .run(); // update and render the view

  view.toImageURL("png", 2).then(function(url) {
    // Remove the first occurrence of the data type header thingy
    const base64res = url.replace("data:image/png;base64,", "");
    const buf = Buffer.from(base64res, "base64")
    callback(buf, 200);
  }).catch(function(error) { 
    callback(null, 500); 
  });
}

exports.plotjson_GCF = function(req, res) {
  plotjson(req.body, req.query, function(output, statusCode) {
    if(statusCode === 200) {
      res.set("content-type", "image/png");
      res.status(200).send(output);
    } else {
      res.set("content-type", "text/plain");
      res.status(statusCode).send(output);
    }
  });
};
