# https://stackoverflow.com/questions/4998173/how-do-i-write-to-standard-error-in-powershell
if (Test-Path -Path 'test_file' -PathType Leaf) {
    # state
    Write-Host -NoNewLine 'exists'

    # display string
    [Console]::Error.WriteLine('`test_file` exists')
}
else {
    # state
    Write-Host -NoNewLine 'not_exists'

    # display string
    [Console]::Error.WriteLine('`test_file` does not exist')
}
