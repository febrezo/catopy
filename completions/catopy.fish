complete -c catopy -l head -d "Copy only first N lines" -r
complete -c catopy -l tail -d "Copy only last N lines" -r
complete -c catopy -l force -d "Bypass size guard"
complete -c catopy -l max-bytes -d "Maximum file size before refusing copy" -r
complete -c catopy -l no-color -d "Disable ANSI colors"
complete -c catopy -l help -d "Print help"
complete -c catopy -l version -d "Print version"
complete -c catopy -f -a "(__fish_complete_path)"
