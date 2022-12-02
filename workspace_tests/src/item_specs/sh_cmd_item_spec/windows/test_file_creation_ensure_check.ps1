$current=$args[0]
$desired=$args[1]
$diff=$args[2]

switch ("$diff")
{
    'creation_required' {
        Write-Host -NoNewLine 'true'
    }
    'exists_sync' {
        Write-Host -NoNewLine 'false'
    }
}
