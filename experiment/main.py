import mlflow
import pathlib
import subprocess
from subprocess import PIPE
import os
import time


class Color:
    BLACK = '\033[30m'  # (文字)黒
    RED = '\033[31m'  # (文字)赤
    GREEN = '\033[32m'  # (文字)緑
    YELLOW = '\033[33m'  # (文字)黄
    BLUE = '\033[34m'  # (文字)青
    MAGENTA = '\033[35m'  # (文字)マゼンタ
    CYAN = '\033[36m'  # (文字)シアン
    WHITE = '\033[37m'  # (文字)白
    COLOR_DEFAULT = '\033[39m'  # 文字色をデフォルトに戻す
    BOLD = '\033[1m'  # 太字
    UNDERLINE = '\033[4m'  # 下線
    INVISIBLE = '\033[08m'  # 不可視
    REVERCE = '\033[07m'  # 文字色と背景色を反転
    BG_BLACK = '\033[40m'  # (背景)黒
    BG_RED = '\033[41m'  # (背景)赤
    BG_GREEN = '\033[42m'  # (背景)緑
    BG_YELLOW = '\033[43m'  # (背景)黄
    BG_BLUE = '\033[44m'  # (背景)青
    BG_MAGENTA = '\033[45m'  # (背景)マゼンタ
    BG_CYAN = '\033[46m'  # (背景)シアン
    BG_WHITE = '\033[47m'  # (背景)白
    BG_DEFAULT = '\033[49m'  # 背景色をデフォルトに戻す
    RESET = '\033[0m'  # 全てリセット


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
