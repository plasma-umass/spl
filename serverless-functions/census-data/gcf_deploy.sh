#!/bin/bash
gcloud functions deploy census-data --region us-east1 --entry-point main --runtime nodejs6 --trigger-http