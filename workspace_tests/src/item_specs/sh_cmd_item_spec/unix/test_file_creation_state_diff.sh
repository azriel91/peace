#! /bin/bash
set -euo pipefail

current=$0
desired=$1

case "$current $desired" in
    'not_exists exists')
        # state
        printf 'creation_required'

        # display string
        printf '`test_file` will be created' 1>&2
        ;;
    'exists exists')
        # state
        printf 'exists_sync'

        # display string
        printf 'nothing to do' 1>&2
        ;;
esac
