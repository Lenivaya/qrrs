_qrrs() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="qrrs"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        qrrs)
            opts="-r -t -o -m -i -h -V --read --terminal --output-format --margin --invert_colors --help --version <INPUT> [OUTPUT]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output-format)
                    COMPREPLY=($(compgen -W "image svg unicode" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "image svg unicode" -- "${cur}"))
                    return 0
                    ;;
                --margin)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _qrrs -o nosort -o bashdefault -o default qrrs
else
    complete -F _qrrs -o bashdefault -o default qrrs
fi
