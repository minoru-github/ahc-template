import mlflow
import pathlib
import subprocess
from subprocess import PIPE
import os
from color import *
import datetime

def prepare_execute_file():
    os.chdir('../')
    cmd = "cargo build -q --release"
    subprocess.run(cmd)
    print("build complete")

    cmd = "copy /y .\\target\\release\\ahc.exe .\\experiment\\ahc.exe"
    subprocess.run(cmd, shell=True)
    os.chdir('./experiment')
    path = pathlib.Path('ahc.exe')
    dt = datetime.datetime.fromtimestamp(path.stat().st_ctime)
    print("updated {}\n".format(dt.strftime('%Y年%m月%d日 %H:%M:%S')))


def parse(proc):
    # 標準エラー出力をパース
    out = proc.stderr.splitlines()
    cnt = int(out[0])
    score = int(out[1])
    duration = float(out[2])
    return cnt, score, duration


def compute_score():
    total_score = 0
    total_cnt = 0
    data_path = "./data/"
    for i, filename in enumerate(pathlib.Path(data_path).glob("*.txt")):
        print(filename)
        cmd = "ahc.exe > ./out/" + filename.name
        path = os.path.join(os.getcwd(), filename)
        with open(path) as text:
            proc = subprocess.run(cmd, shell=True, stdin=text,
                                  stdout=PIPE, stderr=PIPE, text=True)

            cnt, score, duration = parse(proc)
            total_cnt += cnt
            total_score += score

            check_point_col = set_color_to_check_point(i)
            score_col = set_color_to_score(score)

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


if __name__ == "__main__":
    prepare_execute_file()
    total_score = compute_score()
    with mlflow.start_run(run_name="ahc"):
        mlflow.log_metric(key="total score", value=total_score)
