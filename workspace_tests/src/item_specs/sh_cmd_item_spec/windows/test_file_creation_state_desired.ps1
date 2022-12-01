# state
Write-Information 'exists'

# display string
Write-Error -Message '`test_file` exists' -ErrorVariable ev 2>$null
$ev[0].exception
