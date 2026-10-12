#!/usr/bin/env bash
# Fetches Labelary's reference render of every case in compat/cases that has none yet:
# cases/foo.zpl -> cases/foo.labelary.png, plus foo.labelary.2.png ... when the label
# produces more than one label (Labelary's X-Total-Count header).
#
#   compat/fetch.sh [name-filter]
#
# Labelary's free API allows 3 requests/second and 5000/day: this sends at most 2 per
# second and backs off on 429. References are committed, so run it only for new cases.
# Only send synthetic labels from compat/cases.
set -euo pipefail

cd "$(dirname "$0")/cases"
filter="${1:-}"
api="https://api.labelary.com/v1/printers"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# fetch <zpl file> <dpmm> <WxH> <index> <output png>; prints the X-Total-Count.
fetch() {
    local file="$1" dpmm="$2" size="$3" index="$4" out="$5" delay=2 status
    while :; do
        sleep 0.5
        echo >> "$tmp/requests"
        status="$(curl -sS -o "$tmp/body" -D "$tmp/headers" -w '%{http_code}' \
            -H 'Accept: image/png' -H 'X-Quality: Bitonal' \
            -F "file=@$file" "$api/${dpmm}dpmm/labels/$size/$index/")"
        if [ "$status" = 429 ] || [ "${status:0:1}" = 5 ]; then
            echo "  HTTP $status, retrying in ${delay}s" >&2
            sleep "$delay"
            delay=$((delay * 2))
            [ "$delay" -le 64 ] || { echo "  giving up" >&2; return 1; }
            continue
        fi
        if [ "$status" != 200 ]; then
            echo "  HTTP $status: $(head -c 300 "$tmp/body")" >&2
            return 1
        fi
        mv "$tmp/body" "$out"
        tr -d '\r' < "$tmp/headers" | awk -F': *' 'tolower($1) == "x-total-count" { print $2 }'
        return 0
    done
}

for file in *.zpl; do
    name="${file%.zpl}"
    [[ -z "$filter" || "$name" == *"$filter"* ]] || continue
    [ -e "$name.labelary.png" ] && continue

    header="$(grep -aoE 'compat: *[0-9.]+x[0-9.]+( +dpmm=[0-9]+)?' "$file" | head -n 1 || true)"
    size="$(echo "$header" | sed -E 's/compat: *([0-9.]+x[0-9.]+).*/\1/')"
    dpmm="$(echo "$header" | sed -nE 's/.*dpmm=([0-9]+).*/\1/p')"
    dpmm="${dpmm:-8}"
    if [ -z "$header" ]; then
        echo "$file: missing ^FX compat: WxH header" >&2
        continue
    fi

    echo "$name ($size in, ${dpmm}dpmm)"
    total="$(fetch "$file" "$dpmm" "$size" 0 "$tmp/first.png")" || continue
    total="${total:-1}"
    for ((i = 1; i < total; i++)); do
        fetch "$file" "$dpmm" "$size" "$i" "$name.labelary.$((i + 1)).png" > /dev/null || true
    done
    # Label 1 last, so an interrupted run fetches the whole case again.
    mv "$tmp/first.png" "$name.labelary.png"
    [ "$total" -gt 1 ] && echo "  $total labels"
done
echo "$(cat "$tmp/requests" 2>/dev/null | wc -l | tr -d " ") request(s) sent"
