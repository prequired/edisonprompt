
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'edisonprompt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'edisonprompt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'edisonprompt' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new prompt from clipboard or input')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve and render a prompt with variables')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List prompts with optional filtering')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search prompts by content using full-text search')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit an existing prompt in your editor')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete a prompt permanently')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export prompts to JSON format')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import prompts from JSON format')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'edisonprompt;add' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Add tags to the prompt')
            [CompletionResult]::new('--tags', '--tags', [CompletionResultType]::ParameterName, 'Add tags to the prompt')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Use interactive mode for content input')
            [CompletionResult]::new('--interactive', '--interactive', [CompletionResultType]::ParameterName, 'Use interactive mode for content input')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Force overwrite if prompt exists')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Force overwrite if prompt exists')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;get' {
            [CompletionResult]::new('--var', '--var', [CompletionResultType]::ParameterName, 'Variable values in key=value format')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Copy result to clipboard')
            [CompletionResult]::new('--copy', '--copy', [CompletionResultType]::ParameterName, 'Copy result to clipboard')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Output raw content without rendering')
            [CompletionResult]::new('--raw', '--raw', [CompletionResultType]::ParameterName, 'Output raw content without rendering')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;list' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Filter by tag')
            [CompletionResult]::new('--tag', '--tag', [CompletionResultType]::ParameterName, 'Filter by tag')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Limit number of results')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Limit number of results')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Sort by field')
            [CompletionResult]::new('--sort', '--sort', [CompletionResultType]::ParameterName, 'Sort by field')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--names-only', '--names-only', [CompletionResultType]::ParameterName, 'Show only names')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;search' {
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Limit number of results')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Limit number of results')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--highlight', '--highlight', [CompletionResultType]::ParameterName, 'Highlight search terms in results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;edit' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('--yes', '--yes', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;delete' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('--yes', '--yes', [CompletionResultType]::ParameterName, 'Skip confirmation prompt')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Force delete without confirmation')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Force delete without confirmation')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;export' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path (stdout if not specified)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file path (stdout if not specified)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Filter by tag')
            [CompletionResult]::new('--tag', '--tag', [CompletionResultType]::ParameterName, 'Filter by tag')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Pretty print JSON')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'Pretty print JSON')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;import' {
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Input file path (stdin if not specified)')
            [CompletionResult]::new('--input', '--input', [CompletionResultType]::ParameterName, 'Input file path (stdin if not specified)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Merge strategy for existing prompts')
            [CompletionResult]::new('--merge', '--merge', [CompletionResultType]::ParameterName, 'Merge strategy for existing prompts')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run - show what would be imported')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;completions' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Specify custom config file path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'edisonprompt;help' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new prompt from clipboard or input')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve and render a prompt with variables')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List prompts with optional filtering')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search prompts by content using full-text search')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit an existing prompt in your editor')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete a prompt permanently')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export prompts to JSON format')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import prompts from JSON format')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'edisonprompt;help;add' {
            break
        }
        'edisonprompt;help;get' {
            break
        }
        'edisonprompt;help;list' {
            break
        }
        'edisonprompt;help;search' {
            break
        }
        'edisonprompt;help;edit' {
            break
        }
        'edisonprompt;help;delete' {
            break
        }
        'edisonprompt;help;export' {
            break
        }
        'edisonprompt;help;import' {
            break
        }
        'edisonprompt;help;completions' {
            break
        }
        'edisonprompt;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
