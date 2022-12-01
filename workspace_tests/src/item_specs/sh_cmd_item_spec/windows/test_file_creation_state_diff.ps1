$current=$args[0]
$desired=$args[1]

switch ("$current $desired")
{
    'not_exists exists' {
        # state
        Write-Information 'creation_required'

        # display string
        Write-Error -Message '`test_file` will be created' -ErrorVariable ev 2>$null
        $ev[0].exception
    }
    'exists exists' {
        # state
        Write-Information 'exists_sync'

        # display string
        Write-Error -Message 'Nothing to do' -ErrorVariable ev 2>$null
        $ev[0].exception
    }
}
