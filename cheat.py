import numpy as np
from scipy.fft import fft
import wave

wav_file = wave.open("Bongo_sound.wav")
metadata = wav_file.getparams()
frames = wav_file.readframes(metadata.nframes)
wav_file.close()

pcm_samples = np.frombuffer(frames, dtype="<h")
normalized_amplitudes = pcm_samples / (2 ** 15)

# Perform the Fourier Transform on the mystery signal
mystery_signal_fft = fft(normalized_amplitudes)

# Compute the amplitude spectrum
amplitude_spectrum = np.abs(mystery_signal_fft)

# Normalize the amplitude spectrum
amplitude_spectrum = amplitude_spectrum / np.max(amplitude_spectrum)

# Compute the frequency array
freqs = np.fft.fftfreq(num_samples, 1 / metadata.sampling_rate)

# Plot the amplitude spectrum in the frequency domain
plt.plot(freqs[:num_samples // 2], amplitude_spectrum[:num_samples // 2])
plt.xlabel("Frequency [Hz]")
plt.ylabel("Normalized Amplitude")
plt.title("Amplitude Spectrum of the Mystery Signal")
plt.show()

# Find the dominant frequencies
threshold = 0.2
dominant_freq_indices = np.where(amplitude_spectrum[:num_samples // 2] >= threshold)[0]
dominant_freqs = freqs[dominant_freq_indices]

print("Dominant Frequencies: ", dominant_freqs)
