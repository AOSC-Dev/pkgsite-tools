# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_pkgsite_global_optspecs
	string join \n h/help V/version
end

function __fish_pkgsite_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_pkgsite_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_pkgsite_using_subcommand
	set -l cmd (__fish_pkgsite_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c pkgsite -n "__fish_pkgsite_needs_command" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -s V -l version -d 'Print version'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "depends" -d 'Query dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "dep" -d 'Query dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "rdepends" -d 'Query reverse dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "rdep" -d 'Query reverse dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "show" -d 'Get package information'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "info" -d 'Get package information'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "search" -d 'Search for packages'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "updates" -d 'List 100 latest source updates'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "files" -d 'List files for a package'
complete -c pkgsite -n "__fish_pkgsite_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand depends" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand dep" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand rdepends" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand rdep" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand show" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand info" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand search" -s s -l search-only -l no-redir
complete -c pkgsite -n "__fish_pkgsite_using_subcommand search" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand updates" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand files" -s h -l help -d 'Print help'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "depends" -d 'Query dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "rdepends" -d 'Query reverse dependencies of packages'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "show" -d 'Get package information'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "search" -d 'Search for packages'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "updates" -d 'List 100 latest source updates'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "files" -d 'List files for a package'
complete -c pkgsite -n "__fish_pkgsite_using_subcommand help; and not __fish_seen_subcommand_from depends rdepends show search updates files help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
