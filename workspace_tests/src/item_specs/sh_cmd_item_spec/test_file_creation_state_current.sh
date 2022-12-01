#! /bin/bash
set -euo pipefail

if test -f 'test_file'
then
    # state
    printf 'exists'

    # display string
    printf '`test_file` exists' 1>&2
else
    # state
    printf 'not_exists'

    # display string
    printf '`test_file` does not exist' 1>&2
fi
