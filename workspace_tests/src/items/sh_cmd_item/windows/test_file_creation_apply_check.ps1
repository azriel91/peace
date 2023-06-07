$current=$args[0]
$goal=$args[1]
$diff=$args[2]

switch ("$diff")
{
    'not_exists_sync' {
        Write-Host -NoNewLine 'false'
    }
    'creation_required' {
        Write-Host -NoNewLine 'true'
    }
    'deletion_required' {
        Write-Host -NoNewLine 'true'
    }
    'exists_sync' {
        Write-Host -NoNewLine 'false'
    }
}
