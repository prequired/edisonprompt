# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_promptedo_global_optspecs
	string join \n v/verbose c/config= no-color h/help V/version
end

function __fish_promptedo_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_promptedo_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_promptedo_using_subcommand
	set -l cmd (__fish_promptedo_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c promptedo -n "__fish_promptedo_needs_command" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_needs_command" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_needs_command" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c promptedo -n "__fish_promptedo_needs_command" -s V -l version -d 'Print version'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "add" -d 'Add a new prompt from clipboard or input'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "get" -d 'Retrieve and render a prompt with variables'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "list" -d 'List prompts with optional filtering'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "search" -d 'Search prompts by content using full-text search'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "edit" -d 'Edit an existing prompt in your editor'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "delete" -d 'Delete a prompt permanently'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "export" -d 'Export prompts to JSON format'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "import" -d 'Import prompts from JSON format'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "completions" -d 'Generate shell completions'
complete -c promptedo -n "__fish_promptedo_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s t -l tags -d 'Add tags to the prompt' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s i -l interactive -d 'Use interactive mode for content input'
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s f -l force -d 'Force overwrite if prompt exists'
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand add" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -l var -d 'Variable values in key=value format' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -s c -l copy -d 'Copy result to clipboard'
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -s r -l raw -d 'Output raw content without rendering'
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand get" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s t -l tag -d 'Filter by tag' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s f -l format -d 'Output format' -r -f -a "table\t''
json\t''
plain\t''"
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s l -l limit -d 'Limit number of results' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s s -l sort -d 'Sort by field' -r -f -a "name\t''
created\t''
updated\t''"
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -l names-only -d 'Show only names'
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand list" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -s l -l limit -d 'Limit number of results' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -s f -l format -d 'Output format' -r -f -a "table\t''
json\t''
plain\t''"
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -l highlight -d 'Highlight search terms in results'
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand search" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand edit" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand edit" -s y -l yes -d 'Skip confirmation prompt'
complete -c promptedo -n "__fish_promptedo_using_subcommand edit" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand edit" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand edit" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -s y -l yes -d 'Skip confirmation prompt'
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -s f -l force -d 'Force delete without confirmation'
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand delete" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s o -l output -d 'Output file path (stdout if not specified)' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s t -l tag -d 'Filter by tag' -r
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s p -l pretty -d 'Pretty print JSON'
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand export" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -s i -l input -d 'Input file path (stdin if not specified)' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -s m -l merge -d 'Merge strategy for existing prompts' -r -f -a "skip\t''
overwrite\t''
rename\t''"
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -l dry-run -d 'Dry run - show what would be imported'
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand import" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand completions" -s c -l config -d 'Specify custom config file path' -r -F
complete -c promptedo -n "__fish_promptedo_using_subcommand completions" -s v -l verbose -d 'Enable verbose output'
complete -c promptedo -n "__fish_promptedo_using_subcommand completions" -l no-color -d 'Disable colored output'
complete -c promptedo -n "__fish_promptedo_using_subcommand completions" -s h -l help -d 'Print help'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "add" -d 'Add a new prompt from clipboard or input'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "get" -d 'Retrieve and render a prompt with variables'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "list" -d 'List prompts with optional filtering'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "search" -d 'Search prompts by content using full-text search'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "edit" -d 'Edit an existing prompt in your editor'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "delete" -d 'Delete a prompt permanently'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "export" -d 'Export prompts to JSON format'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "import" -d 'Import prompts from JSON format'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "completions" -d 'Generate shell completions'
complete -c promptedo -n "__fish_promptedo_using_subcommand help; and not __fish_seen_subcommand_from add get list search edit delete export import completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
