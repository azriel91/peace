$current=$args[0]
$desired=$args[1]

switch ("$current $desired")
{
    'not_exists not_exists' {
        # state
        Write-Host -NoNewLine 'not_exists_sync'

        # display string
        [Console]::Error.WriteLine('`test_file` does not exist')
    }
    'not_exists exists' {
        # state
        Write-Host -NoNewLine 'creation_required'

        # display string
        [Console]::Error.WriteLine('`test_file` will be created')
    }
    'exists not_exists' {
        # state
        Write-Host -NoNewLine 'deletion_required'

        # display string
        [Console]::Error.WriteLine('`test_file` will be deleted')
    }
    'exists exists' {
        # state
        Write-Host -NoNewLine 'exists_sync'

        # display string
        [Console]::Error.WriteLine('nothing to do')
    }
}
