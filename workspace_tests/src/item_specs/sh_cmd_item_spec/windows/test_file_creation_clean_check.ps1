$current=$args[0]

switch ("$current")
{
    'not_exists' {
        Write-Information 'false'
    }
    'exists' {
        Write-Information 'true'
    }
}
