#! /bin/bash
#
# Script to download the files used in this theme.
set -euo pipefail

curl https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js -o highlight.js
curl https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/tomorrow-night-bright.min.css -o highlight.css
