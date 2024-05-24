import matplotlib.pyplot as plt
from scipy.fftpack import fft
from scipy.io import wavfile
import numpy as np

sr, signal = wavfile.read('test.wav')
print(signal.shape, sr)

Signal = fft(signal)

fig, (axt, axf) = plt.subplots(2, 1, constrained_layout=1, figsize=(11.8,3))

axt.plot(signal, lw=0.15)
axt.grid(1)

axf.plot(np.abs(Signal[:sr//2]), lw=0.5)
axf.grid(1)

plt.show()