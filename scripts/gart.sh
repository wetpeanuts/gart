#!/usr/bin/env bash

print_help_message() {
    cat <<EOF
Usage: gart [OPTIONS] <COMMIT_HASH>

Options:
  -h, --help      Show help message
  -V, --version   Show version

Arguments:
  COMMIT_HASH     Commit hash to rebase to
EOF
}

: "${GART_EDITOR:=gart_editor}"
: "${GART_SEQUENCE_EDITOR:=gart_seq_editor}"

case "${1:-}" in
    -h|--help)
        print_help_message
        exit 0
        ;;
    -V|--version)
        echo "gart 0.1.0"
        exit 0
        ;;
    "")
        echo "Missing commit hash" >&2
        print_help_message
        exit 1
        ;;
    *)
        COMMIT_HASH="$1"
        ;;
esac

# Verify git is installed
if ! git --version >/dev/null 2>&1; then
    echo "git is not found"
    exit 1
fi

# Verify commit hash is valid
if ! git rev-parse --verify "$COMMIT_HASH^{commit}" >/dev/null 2>&1; then
    echo "Commit '$COMMIT_HASH' does not exist"
    exit 1
fi

# Verify gart_editor is installed
if ! $GART_EDITOR --version >/dev/null 2>&1; then
    echo "$GART_EDITOR is not found"
    exit 1
fi

# Verify gart_seq_editor is installed
if ! $GART_SEQUENCE_EDITOR --version >/dev/null 2>&1; then
    echo "$GART_SEQUENCE_EDITOR is not found"
    exit 1
fi

GIT_EDITOR=$GART_EDITOR \
    GIT_SEQUENCE_EDITOR=$GART_SEQUENCE_EDITOR \
    git rebase -i $COMMIT_HASH
