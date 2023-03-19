#! /bin/bash
set -euo pipefail

# state
printf 'not_exists'

# display string
printf '`test_file` does not exist' 1>&2
