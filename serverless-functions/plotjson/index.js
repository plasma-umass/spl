"use strict";

const vega = require("vega");


function plotjson(jsonBody, getQuery) {
  // NOTE(arjun): Supporting query parameters and the body will require a bunch
  // of needless engineering. I suggest we either include the names as part
  // of the body, or assume that they are named "x" and "y"
  const xName = (getQuery.xname === undefined) ? "x" : getQuery.xname,
        yName = (getQuery.yname === undefined) ? "y" : getQuery.yname;

  // Validate that JSON is an array
  if(!Array.isArray(jsonBody)) {
    return Promise.reject({message: "JSON input is not an array, instead we received: " + JSON.stringify(jsonBody), status: 400});
  }


  var renamedData = [];
  for(const pair of jsonBody) {

    if((xName in pair) && (yName in pair)) {
      const renamedPair = {x : pair[xName], y : pair[yName]};
      renamedData.push(renamedPair);
    } else {
      return Promise.reject({
        message: `JSON input pair ${JSON.stringify(pair)} does not contain both required keys "${xName}" and "${yName}".`, 
        status: 400});
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
        "title": xName,
        "labels" : false,
        "ticks" : false
      },
      {
        "orient": "left",
        "scale": "yscale",
        "title": yName,
      }
    ],
    "marks": [
      {
        "type": "symbol",
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
            },
            "shape": {"value": "circle"}
          }
        }
      }
    ]
  };

  var view = new vega.View(vega.parse(plot_spec))
      .logLevel(vega.Warn) // set view logging level
      .renderer("svg") // set render type (defaults to "canvas")
      .run(); // update and render the view

  return view.toImageURL("png", 2).then(function(url) {
    // Remove the first occurrence of the data type header thingy
    const base64res = url.replace("data:image/png;base64,", "");
    return Buffer.from(base64res, "base64");
  }).catch(function(error) { 
    return {message: error, status: 500};
  });
}

exports.plotjson_GCF = function(req, res) {
  plotjson(req.body, req.query).then(function (output) {
    res.set("content-type", "image/png");
    res.status(200).send(output);
  }, function(ret) {
    res.set("content-type", "text/plain");
    res.status(ret.status).send(ret.message);
  });
};




