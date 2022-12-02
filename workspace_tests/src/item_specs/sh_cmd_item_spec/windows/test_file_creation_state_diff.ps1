$current=$args[0]
$desired=$args[1]

switch ("$current $desired")
{
    'not_exists exists' {
        # state
        Write-Host -NoNewLine 'creation_required'

        # display string
        [Console]::Error.WriteLine('`test_file` will be created')
    }
    'exists exists' {
        # state
        Write-Host -NoNewLine 'exists_sync'

        # display string
        [Console]::Error.WriteLine('nothing to do')
    }
}
