#!/bin/bash
gcloud functions deploy wait10s --region us-east1 --entry-point wait10s_GCF --runtime nodejs6 --trigger-http