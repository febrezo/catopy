_catopy()
{
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    case "$prev" in
        --head|--tail)
            COMPREPLY=( $(compgen -W "10 20 50 100" -- "$cur") )
            return 0
            ;;
        --max-bytes)
            COMPREPLY=( $(compgen -W "1K 5M 10M 100M 1G" -- "$cur") )
            return 0
            ;;
    esac

    COMPREPLY=( $(compgen -W "--head --tail --force --max-bytes --no-color --help --version" -- "$cur") )
}

complete -F _catopy catopy
