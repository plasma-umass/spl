#!/bin/bash
gcloud functions deploy plotjson --region us-east1 --entry-point plotjson_GCF --runtime nodejs6 --trigger-http