#compdef edisonprompt

autoload -U is-at-least

_edisonprompt() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_edisonprompt_commands" \
"*::: :->edisonprompt" \
&& ret=0
    case $state in
    (edisonprompt)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:edisonprompt-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
'*-t+[Add tags to the prompt]:TAGS:_default' \
'*--tags=[Add tags to the prompt]:TAGS:_default' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-i[Use interactive mode for content input]' \
'--interactive[Use interactive mode for content input]' \
'-f[Force overwrite if prompt exists]' \
'--force[Force overwrite if prompt exists]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Prompt name (alphanumeric, hyphens, underscores only):_default' \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" : \
'*--var=[Variable values in key=value format]:VARIABLES:_default' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-c[Copy result to clipboard]' \
'--copy[Copy result to clipboard]' \
'-r[Output raw content without rendering]' \
'--raw[Output raw content without rendering]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Prompt name to retrieve:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-t+[Filter by tag]:TAG:_default' \
'--tag=[Filter by tag]:TAG:_default' \
'-f+[Output format]:FORMAT:(table json plain)' \
'--format=[Output format]:FORMAT:(table json plain)' \
'-l+[Limit number of results]:LIMIT:_default' \
'--limit=[Limit number of results]:LIMIT:_default' \
'-s+[Sort by field]:SORT:(name created updated)' \
'--sort=[Sort by field]:SORT:(name created updated)' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'--names-only[Show only names]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'-l+[Limit number of results]:LIMIT:_default' \
'--limit=[Limit number of results]:LIMIT:_default' \
'-f+[Output format]:FORMAT:(table json plain)' \
'--format=[Output format]:FORMAT:(table json plain)' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'--highlight[Highlight search terms in results]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':query -- Search query:_default' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-y[Skip confirmation prompt]' \
'--yes[Skip confirmation prompt]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Prompt name to edit:_default' \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-y[Skip confirmation prompt]' \
'--yes[Skip confirmation prompt]' \
'-f[Force delete without confirmation]' \
'--force[Force delete without confirmation]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Prompt name to delete:_default' \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
'-o+[Output file path (stdout if not specified)]:OUTPUT:_files' \
'--output=[Output file path (stdout if not specified)]:OUTPUT:_files' \
'-t+[Filter by tag]:TAG:_default' \
'--tag=[Filter by tag]:TAG:_default' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-p[Pretty print JSON]' \
'--pretty[Pretty print JSON]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(import)
_arguments "${_arguments_options[@]}" : \
'-i+[Input file path (stdin if not specified)]:INPUT:_files' \
'--input=[Input file path (stdin if not specified)]:INPUT:_files' \
'-m+[Merge strategy for existing prompts]:MERGE:(skip overwrite rename)' \
'--merge=[Merge strategy for existing prompts]:MERGE:(skip overwrite rename)' \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'--dry-run[Dry run - show what would be imported]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" : \
'-c+[Specify custom config file path]:FILE:_files' \
'--config=[Specify custom config file path]:FILE:_files' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':shell -- Shell type:(bash zsh fish powershell)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_edisonprompt__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:edisonprompt-help-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(import)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_edisonprompt_commands] )) ||
_edisonprompt_commands() {
    local commands; commands=(
'add:Add a new prompt from clipboard or input' \
'get:Retrieve and render a prompt with variables' \
'list:List prompts with optional filtering' \
'search:Search prompts by content using full-text search' \
'edit:Edit an existing prompt in your editor' \
'delete:Delete a prompt permanently' \
'export:Export prompts to JSON format' \
'import:Import prompts from JSON format' \
'completions:Generate shell completions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'edisonprompt commands' commands "$@"
}
(( $+functions[_edisonprompt__add_commands] )) ||
_edisonprompt__add_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt add commands' commands "$@"
}
(( $+functions[_edisonprompt__completions_commands] )) ||
_edisonprompt__completions_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt completions commands' commands "$@"
}
(( $+functions[_edisonprompt__delete_commands] )) ||
_edisonprompt__delete_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt delete commands' commands "$@"
}
(( $+functions[_edisonprompt__edit_commands] )) ||
_edisonprompt__edit_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt edit commands' commands "$@"
}
(( $+functions[_edisonprompt__export_commands] )) ||
_edisonprompt__export_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt export commands' commands "$@"
}
(( $+functions[_edisonprompt__get_commands] )) ||
_edisonprompt__get_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt get commands' commands "$@"
}
(( $+functions[_edisonprompt__help_commands] )) ||
_edisonprompt__help_commands() {
    local commands; commands=(
'add:Add a new prompt from clipboard or input' \
'get:Retrieve and render a prompt with variables' \
'list:List prompts with optional filtering' \
'search:Search prompts by content using full-text search' \
'edit:Edit an existing prompt in your editor' \
'delete:Delete a prompt permanently' \
'export:Export prompts to JSON format' \
'import:Import prompts from JSON format' \
'completions:Generate shell completions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'edisonprompt help commands' commands "$@"
}
(( $+functions[_edisonprompt__help__add_commands] )) ||
_edisonprompt__help__add_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help add commands' commands "$@"
}
(( $+functions[_edisonprompt__help__completions_commands] )) ||
_edisonprompt__help__completions_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help completions commands' commands "$@"
}
(( $+functions[_edisonprompt__help__delete_commands] )) ||
_edisonprompt__help__delete_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help delete commands' commands "$@"
}
(( $+functions[_edisonprompt__help__edit_commands] )) ||
_edisonprompt__help__edit_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help edit commands' commands "$@"
}
(( $+functions[_edisonprompt__help__export_commands] )) ||
_edisonprompt__help__export_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help export commands' commands "$@"
}
(( $+functions[_edisonprompt__help__get_commands] )) ||
_edisonprompt__help__get_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help get commands' commands "$@"
}
(( $+functions[_edisonprompt__help__help_commands] )) ||
_edisonprompt__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help help commands' commands "$@"
}
(( $+functions[_edisonprompt__help__import_commands] )) ||
_edisonprompt__help__import_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help import commands' commands "$@"
}
(( $+functions[_edisonprompt__help__list_commands] )) ||
_edisonprompt__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help list commands' commands "$@"
}
(( $+functions[_edisonprompt__help__search_commands] )) ||
_edisonprompt__help__search_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt help search commands' commands "$@"
}
(( $+functions[_edisonprompt__import_commands] )) ||
_edisonprompt__import_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt import commands' commands "$@"
}
(( $+functions[_edisonprompt__list_commands] )) ||
_edisonprompt__list_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt list commands' commands "$@"
}
(( $+functions[_edisonprompt__search_commands] )) ||
_edisonprompt__search_commands() {
    local commands; commands=()
    _describe -t commands 'edisonprompt search commands' commands "$@"
}

if [ "$funcstack[1]" = "_edisonprompt" ]; then
    _edisonprompt "$@"
else
    compdef _edisonprompt edisonprompt
fi
