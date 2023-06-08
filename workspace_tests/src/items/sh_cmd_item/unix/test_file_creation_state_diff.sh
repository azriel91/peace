#! /bin/bash
set -euo pipefail

current=$0
goal=$1

case "$current $goal" in
    'not_exists not_exists')
        # state
        printf 'not_exists_sync'

        # display string
        printf '`test_file` does not exist' 1>&2
        ;;
    'not_exists exists')
        # state
        printf 'creation_required'

        # display string
        printf '`test_file` will be created' 1>&2
        ;;
    'exists not_exists')
        # state
        printf 'deletion_required'

        # display string
        printf '`test_file` will be deleted' 1>&2
        ;;
    'exists exists')
        # state
        printf 'exists_sync'

        # display string
        printf 'nothing to do' 1>&2
        ;;
esac
