
use builtin;
use str;

set edit:completion:arg-completer[pkgsite] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'pkgsite'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'pkgsite'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand depends 'Query dependencies of packages'
            cand dep 'Query dependencies of packages'
            cand rdepends 'Query reverse dependencies of packages'
            cand rdep 'Query reverse dependencies of packages'
            cand show 'Get package information'
            cand info 'Get package information'
            cand search 'Search for packages'
            cand updates 'List 100 latest source updates'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pkgsite;depends'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;dep'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;rdepends'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;rdep'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;show'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;info'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;search'= {
            cand -s 's'
            cand --search-only 'search-only'
            cand --no-redir 'search-only'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;updates'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pkgsite;help'= {
            cand depends 'Query dependencies of packages'
            cand rdepends 'Query reverse dependencies of packages'
            cand show 'Get package information'
            cand search 'Search for packages'
            cand updates 'List 100 latest source updates'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pkgsite;help;depends'= {
        }
        &'pkgsite;help;rdepends'= {
        }
        &'pkgsite;help;show'= {
        }
        &'pkgsite;help;search'= {
        }
        &'pkgsite;help;updates'= {
        }
        &'pkgsite;help;help'= {
        }
    ]
    $completions[$command]
}
