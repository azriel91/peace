$current=$args[0]
$desired=$args[1]
$diff=$args[2]

switch ("$diff")
{
    'creation_required' {
        Write-Information 'true'
    }
    'exists_sync' {
        Write-Information 'false'
    }
}
