#! /bin/bash
#
# Script to download the files used in this theme.
set -euo pipefail

curl https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js -o highlight.js
curl https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/tomorrow-night-bright.min.css -o highlight.css

sd -s '3387cc' '55a7ee' highlight.css
sd -s '8996a8' 'a2a8ca' highlight.css
sd -s '65b042' '87d264' highlight.css
sd -s 'e28964' 'e26489' highlight.css
