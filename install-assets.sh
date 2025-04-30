#!/bin/bash -ex

PREFIX="${PREFIX:-/usr/local}"

# install completions
install -dv "${PREFIX}/share/zsh/functions/Completion/Linux/"
install -Dvm644 completions/_pkgsite "${PREFIX}/share/zsh/functions/Completion/Linux/"
install -dv "${PREFIX}/share/fish/vendor_completions.d/"
install -Dvm644 completions/pkgsite.fish "${PREFIX}/share/fish/vendor_completions.d/"
install -dv "${PREFIX}/share/bash-completion/completions/"
install -Dvm644 completions/pkgsite.bash "${PREFIX}/share/bash-completion/completions/"
