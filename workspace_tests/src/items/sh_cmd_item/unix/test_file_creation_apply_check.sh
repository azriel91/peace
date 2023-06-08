#! /bin/bash
set -euo pipefail

current=$0
goal=$1
diff=$2

case "$diff" in
    'not_exists_sync')
        printf 'false'
        ;;
    'creation_required')
        printf 'true'
        ;;
    'deletion_required')
        printf 'true'
        ;;
    'exists_sync')
        printf 'false'
        ;;
esac
