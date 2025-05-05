
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'pkgsite' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'pkgsite'
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
        'pkgsite' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('depends', 'depends', [CompletionResultType]::ParameterValue, 'Query dependencies of packages')
            [CompletionResult]::new('dep', 'dep', [CompletionResultType]::ParameterValue, 'Query dependencies of packages')
            [CompletionResult]::new('rdepends', 'rdepends', [CompletionResultType]::ParameterValue, 'Query reverse dependencies of packages')
            [CompletionResult]::new('rdep', 'rdep', [CompletionResultType]::ParameterValue, 'Query reverse dependencies of packages')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Get package information')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Get package information')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for packages')
            [CompletionResult]::new('updates', 'updates', [CompletionResultType]::ParameterValue, 'List 100 latest source updates')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'pkgsite;depends' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;dep' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;rdepends' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;rdep' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;show' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;info' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;search' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--search-only', '--search-only', [CompletionResultType]::ParameterName, 'search-only')
            [CompletionResult]::new('--no-redir', '--no-redir', [CompletionResultType]::ParameterName, 'search-only')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;updates' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'pkgsite;help' {
            [CompletionResult]::new('depends', 'depends', [CompletionResultType]::ParameterValue, 'Query dependencies of packages')
            [CompletionResult]::new('rdepends', 'rdepends', [CompletionResultType]::ParameterValue, 'Query reverse dependencies of packages')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Get package information')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for packages')
            [CompletionResult]::new('updates', 'updates', [CompletionResultType]::ParameterValue, 'List 100 latest source updates')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'pkgsite;help;depends' {
            break
        }
        'pkgsite;help;rdepends' {
            break
        }
        'pkgsite;help;show' {
            break
        }
        'pkgsite;help;search' {
            break
        }
        'pkgsite;help;updates' {
            break
        }
        'pkgsite;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
