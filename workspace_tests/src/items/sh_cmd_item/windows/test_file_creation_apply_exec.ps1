$current=$args[0]
$goal=$args[1]
$diff=$args[2]

switch ("$diff")
{
    'not_exists_sync' {
        # Nothing to do
    }
    'creation_required' {
        New-Item -Path 'test_file'
    }
    'deletion_required' {
        Remove-Item -Path 'test_file'
    }
    'exists_sync' {
        # Nothing to do
    }
}

