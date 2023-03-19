#! /bin/bash
set -euo pipefail

current=$0
desired=$1
diff=$2

case "$diff" in
    'not_exists_sync')
        # Nothing to do
        ;;
    'creation_required')
        touch 'test_file'
        ;;
    'deletion_required')
        rm -f 'test_file'
        ;;
    'exists_sync')
        # Nothing to do
        ;;
esac
