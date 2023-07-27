import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

df = pd.read_csv("output.csv")
plt.figure(figsize=(12,6), dpi=100)
sns.histplot(data=df, bins=100, color='#7C73C0', kde=True)
plt.title("Monte Carlo Price Distribution")
plt.xlabel("Price", labelpad=10)
plt.ylabel("Density", labelpad=10)
plt.tight_layout()
plt.savefig("price_dist.png")
plt.show()

