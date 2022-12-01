#! /bin/bash
set -euo pipefail

current=$0
desired=$1
diff=$2

case "$diff" in
    'creation_required')
        printf 'true'
        ;;
    'exists_sync')
        printf 'false'
        ;;
esac
