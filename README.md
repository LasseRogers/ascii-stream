# ASCII video over TCP

Streams video as coloured ASCII to terminals and browsers. Frames are sent as
cell deltas; late joiners get a keyframe. Wire format: [PROTOCOL.md](PROTOCOL.md).

- `server/` Rust server and terminal client
- `web/` TypeScript browser client
- `lab/` Python + Numba reference encoder and experiments
- `testdata/` golden vectors shared by all three

## Run

```sh
./run.sh clip.mp4                               # server + http://localhost:5173
cd server && cargo run --release --bin client   # terminal client
```
