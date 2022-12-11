#!/usr/bin/env bash
set -euo pipefail

WHERE="${1?Pass a webilds subdir as the first argument}"
FILE="${2?Specify the file to import as the second argument (i.e. Window.webidl)}"

BASE_URL="https://raw.githubusercontent.com/mozilla/gecko-dev/master/dom/webidl"

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

curl -o "$SCRIPT_DIR/webidls/$WHERE/$FILE" "$BASE_URL/$FILE"
