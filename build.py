import argparse
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("n", help="the number of build to do", type=int)
args = parser.parse_args()

start_at = time.time()

for i in range(args.n):
    print(f"Build {i+1}/{args.n}")
    p = subprocess.run(
        ["cargo", "tauri", "build"],
        text=True,
        capture_output=True
    )
    if p.returncode != 0:
        print(p.stderr)
        print(f"Build {i+1}/{args.n} failed")
        break

print(f"Total time: {time.time() - start_at:.2f}s")
