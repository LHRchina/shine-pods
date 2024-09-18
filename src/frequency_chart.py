import librosa
import numpy as np
import matplotlib.pyplot as plt


def plot_frequency_chart(audio_file):

    # Load the audio file
    audio_data, sampling_rate = librosa.load(audio_file)
    # Compute the FFT
    fft_spectrum = np.fft.fft(audio_data)

    # Compute the frequencies corresponding to the FFT values
    frequency = np.fft.fftfreq(len(fft_spectrum), d=1/sampling_rate)
    # Compute the magnitude spectrum
    amplitude_spectrum = np.abs(fft_spectrum)
    # Only use the positive frequencies
    half_length = len(frequency) // 2
    frequencies = frequency[:half_length]
    amplitudes = amplitude_spectrum[:half_length]

    # Plot the frequency spectrum
    plt.figure(figsize=(12, 6))
    plt.plot(frequencies, amplitudes)
    plt.title('Frequency Spectrum')
    plt.xlabel('Frequency (Hz)')
    plt.ylabel('Amplitude')
    plt.grid(True)
    plt.show()

    # Plot the frequency chart
    plt.figure(figsize=(10, 6))
    plt.plot(frequency, amplitude_spectrum)
    plt.title('Frequency Chart')
    plt.xlabel('Frequency (Hz)')
    plt.ylabel('Amplitude')


    

if __name__ == "__main__":
    plot_frequency_chart("store/unreachable.mp3")


    