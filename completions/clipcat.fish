complete -c clipcat -l head -d "Copy only first N lines" -r
complete -c clipcat -l tail -d "Copy only last N lines" -r
complete -c clipcat -l force -d "Bypass size guard"
complete -c clipcat -l max-bytes -d "Maximum file size before refusing copy" -r
complete -c clipcat -l no-color -d "Disable ANSI colors"
complete -c clipcat -l help -d "Print help"
complete -c clipcat -l version -d "Print version"
complete -c clipcat -f -a "(__fish_complete_path)"
