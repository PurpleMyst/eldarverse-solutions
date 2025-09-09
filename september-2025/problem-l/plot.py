# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "matplotlib",
#     "pandas",
#     "seaborn",
# ]
# ///

# read full.csv and make scatterplot
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

def main() -> None:
    df = pd.read_csv("full.csv")
    plt.figure(figsize=(10, 6))
    sns.scatterplot(data=df, x="x", y="y", s=100)
    # overlay log2
    # for i in range(2, 16):
    #     df[f"log_{i}"] = df["x"].apply(lambda x: np.log(x) / np.log(i) if x > 0 else np.nan)
    #     sns.scatterplot(data=df, x="x", y=f"log_{i}", legend="full")
    plt.title("Scatter Plot of full.csv")
    plt.xlabel("X-axis")
    plt.ylabel("Y-axis")
    plt.grid(True)
    plt.savefig("scatter_plot.png")
    plt.show()


if __name__ == "__main__":
    main()
