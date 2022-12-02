$current=$args[0]
$desired=$args[1]

switch ("$current $desired")
{
    'not_exists exists' {
        # state
        Write-Information 'creation_required'

        # display string
        [Console]::Error.WriteLine('`test_file` will be created')
    }
    'exists exists' {
        # state
        Write-Information 'exists_sync'

        # display string
        [Console]::Error.WriteLine('Nothing to do')
    }
}
