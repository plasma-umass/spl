'use strict';

const exec = require('child_process').exec;
const vega = require('vega');

/**
 *
 * @param {Object} req A Javascript object of the JSON body, or null if there is no parsable JSON body
 * @param {Object} res A Javascript object of the GET query.
 */
function plotjson(jsonBody, getQuery, callback) {
  const xName = getQuery.xname;
  const yName = getQuery.yname;

  // Validate that JSON is an array
  if(!Array.isArray(jsonBody)) {
    callback(null);
    return;
  }

  // Remap the JSON input by renaming xName => "x", yName => "y"
  var mapFailed = false;
  const renamedData = jsonBody.map(pair => {
    if((xName in pair) && (yName in pair)) {
      const renamedPair = {"x" : pair[xName], "y" : pair[yName]};
      return renamedPair
    } else {
      callback(null);
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

  const parsed = vega.parse(plot_spec);

  var view = new vega.View(parsed)
    .logLevel(vega.Warn) // set view logging level
    .renderer('svg') // set render type (defaults to 'canvas')
    .run(); // update and render the view

  view.toImageURL('png', 2).then(function(url) {
    // Remove the first occurrence of the data type header thingy
    const base64res = url.replace("data:image/png;base64,", "");
    const buf = Buffer.from(base64res, 'base64')
    callback(buf);
  }).catch(function(error) { /* error handling */ });
}

exports.plotjson_GCF = (req, res) => {
  plotjson(req.body, req.query, output => {
    res.status(200);
    res.set('Content-Type', 'image/png');
    res.send(output);
    res.end();
  });
};
