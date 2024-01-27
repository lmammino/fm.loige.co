#!/usr/bin/env bash

set -v

sam build

sam deploy \
  --stack-name loige-fm \
  --parameter-overrides "ParameterKey=LastFmApiKey,ParameterValue=${LASTFM_API_KEY}" \
  --no-confirm-changeset