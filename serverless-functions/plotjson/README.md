# plotjson

`plotjson` accepts JSON input to plot, along with URL parameters indicating which columns are for the `x` and `y`. Then, `plotjson` will return the binary data for a PNG image of the data plotted as a line plot. An example command to this this:

```bash
curl -X POST 'https://us-central1-cloudfun-mapreduce.cloudfunctions.net/plotjson_GCF?xname=age&yname=weight' -H "Content-Type:application/json"  -d '[{"age":4, "weight":8}, {"age":5, "weight":9}, {"age":6, "weight":11}, {"age":7, "weight":4.5}]' > plot.png
```

after which you can inspect `plot.png`.