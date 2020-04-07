import ser_parser
import matplotlib.pyplot as plt
import numpy as np

shape, data = ser_parser.parser("test_data/-3_1.ser")

plt.imshow(np.array(data).reshape(shape))
plt.show()
