#!/usr/bin/env bash
# Stream a video: server on :7878/:7879, browser client on http://localhost:5173.
set -euo pipefail
cd "$(dirname "$0")"

video=$(realpath "${1:?usage: ./run.sh video.mp4}")
ln -sf "$video" web/public/source.mp4

trap 'kill 0' EXIT
(cd server && cargo run --release --bin server -- "$video") &
(cd web && npm install --silent && npm run dev) &
wait
