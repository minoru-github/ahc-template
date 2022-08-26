import mlflow
import pathlib
import subprocess
from subprocess import PIPE
import os
import time

def compute_score():
    total = 0
    data_path = "./data/"
    os.chdir('../')
    for filename in pathlib.Path(data_path).glob("*.txt"):
        cmd = "cargo run -q --release --bin ahc > ./out/" + filename.name
        path = os.path.join(os.getcwd(), filename)
        with open(path) as text:
            time_start = time.time()
            proc = subprocess.run(cmd, shell=True, stdin=text,
                                stdout=PIPE, stderr=PIPE, text=True)
            time_end = time.time()
            elapsed = time_end-time_start

            score = int(proc.stderr)
            print("{} => score: {}, time: {}".format(filename, score, elapsed))
            total += score

    print("total: {}".format(total))
    return total

# with mlflow.start_run(run_name='2'):
#     for epoch in range(0, 5):
#         mlflow.log_metric(key="train acc", value = 2*epoch, step=epoch)

score = compute_score()
os.chdir('./experiment')
with mlflow.start_run(run_name="ahc"):
    mlflow.log_metric(key="total score", value=score)
