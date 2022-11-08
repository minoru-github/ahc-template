import mlflow
import pathlib
import subprocess
from subprocess import PIPE
import os
import time
from color import Color

def compute_score():
    total_score = 0
    total_cnt = 0
    data_path = "./data/"
    os.chdir('../')
    for i, filename in enumerate(pathlib.Path(data_path).glob("*.txt")):
        cmd = "cargo run -q --release --bin ahc > ./out/" + filename.name
        path = os.path.join(os.getcwd(), filename)
        with open(path) as text:
            proc = subprocess.run(cmd, shell=True, stdin=text,
                                  stdout=PIPE, stderr=PIPE, text=True)

            # 標準エラー出力をパース
            out = proc.stderr.splitlines()
            cnt = int(out[0])
            total_cnt += cnt
            score = int(out[1])
            total_score += score
            duration = float(out[2])

            check_point_col = Color.BG_DEFAULT
            if (i+1) % 5 == 0:
                check_point_col = Color.MAGENTA

            score_col = Color.BG_DEFAULT
            if score < 700000:
                score_col = Color.BLUE
            elif score < 800000:
                score_col = Color.GREEN
            elif score < 900000:
                score_col = Color.YELLOW
            elif score < 1000000:
                score_col = Color.MAGENTA
            else:
                score_col = Color.RED

            print("{} => ".format(filename.name[:4])
                  + "total_score: {}{}{}, ".format(check_point_col,
                                                   total_score, Color.RESET)
                  + "score: {}{}{}, ".format(score_col, score, Color.RESET)
                  + "cnt: {:5d}, ".format(cnt)
                  + "total_cnt: {}{}{}, " .format(check_point_col,
                                                  total_cnt, Color.RESET)
                  + "time: {:.3f}".format(duration))

    print("total: {}".format(total_score))
    return total_score

# with mlflow.start_run(run_name='2'):
#     for epoch in range(0, 5):
#         mlflow.log_metric(key="train acc", value = 2*epoch, step=epoch)


score = compute_score()
os.chdir('./experiment')
with mlflow.start_run(run_name="ahc"):
    mlflow.log_metric(key="total score", value=score)
