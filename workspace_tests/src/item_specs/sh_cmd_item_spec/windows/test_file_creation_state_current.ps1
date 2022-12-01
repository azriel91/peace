# https://stackoverflow.com/questions/4998173/how-do-i-write-to-standard-error-in-powershell
if (Test-Path -Path 'test_file' -PathType Leaf) {
    # state
    Write-Information 'exists'

    # display string
    Write-Error -Message '`test_file` exists' -ErrorVariable ev 2>$null
    $ev[0].exception
}
else {
    # state
    Write-Information 'not_exists'

    # display string
    Write-Error -Message '`test_file` does not exist' -ErrorVariable ev 2>$null
    $ev[0].exception
}
