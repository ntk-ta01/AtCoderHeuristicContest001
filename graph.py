import matplotlib.pyplot as plt

temp = []
val = []
with open("./plot.out") as f:
    for line in f:
        a, b = (float(i) for i in line.split())
        temp.append(a)
        val.append(b)

# plt.rcParams['font.family'] = "MS Gothic"

fig = plt.figure()
ax = fig.add_subplot(111)

ax.plot(temp, val)
# ax.invert_xaxis()
ax.set_xlabel("temp", size=14, weight="light")
ax.set_ylabel("objective function value", size=14, weight="light")
ax.set_xticks([10000000 - 999000*x for x in range(11)])
ax.set_yticks([max(val), sum(val) / len(val), min(val)])
ax.invert_xaxis()
ax.grid(which="major", axis="x", color="black", alpha=0.25,
        linestyle="--", linewidth=1)
ax.grid(which="major", axis="y", color="black", alpha=0.25,
        linestyle="--", linewidth=1)
plt.savefig("graph.png")
