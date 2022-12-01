#! /bin/bash
set -euo pipefail

current=$0

case "$current" in
    'not_exists')
        printf 'false'
        ;;
    'exists')
        printf 'true'
        ;;
esac
