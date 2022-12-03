$current=$args[0]

switch ("$current")
{
    'not_exists' {
        Write-Host -NoNewLine 'false'
    }
    'exists' {
        Write-Host -NoNewLine 'true'
    }
}
