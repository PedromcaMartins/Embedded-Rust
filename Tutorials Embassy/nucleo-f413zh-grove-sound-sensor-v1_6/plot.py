import matplotlib.pyplot as plt

# File name containing the data
file_name = "COM3_2024_08_20.00.15.56.007.txt"

# Reading the data from the file
with open(file_name, 'r') as file:
    data = [int(line.strip()) for line in file]

# Plotting the data
plt.figure(figsize=(12, 6))
plt.plot(data, marker='o', linestyle='-', color='b')
plt.title('Data Plot')
plt.xlabel('Index')
plt.ylabel('Value')
plt.grid(True)
plt.show()
