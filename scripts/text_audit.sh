#!/bin/sh
# Fail if any tracked-or-untracked text file in the repo contains an en dash
# (U+2013), an em dash (U+2014), or one of the two quarantined terms used by
# the fixture's verify_chimerax_interop.py. The terms are split here, as the
# verifier splits them, so this script does not match itself.
set -u
cd "$(dirname "$0")/.."

t1="radiotrophic"" methanogens"
t2="free-will"" octane"

hits=$(grep -rIl --exclude-dir=.agents --exclude-dir=target --exclude-dir=.build --exclude-dir=.git \
    --exclude-dir='*.xcframework' -e "$(printf '\342\200\223')" -e "$(printf '\342\200\224')" \
    -i -e "$t1" -e "$t2" . 2>/dev/null)

if [ -n "$hits" ]; then
    echo "text audit FAILED:"
    echo "$hits"
    exit 1
fi
echo "text audit passed: 0 en dashes, 0 em dashes, 0 quarantined terms"
