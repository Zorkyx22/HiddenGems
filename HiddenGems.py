import numpy as np
import numpy.typing as npt
import ffmpeg
from PIL import Image

flag = 'ULCTF-{This-is-my-32-characters-Flags!!}'

def string_to_binary_array(string):
    bin_chars = [format(ord(c), '08b') for c in string]
    chars = []
    for bin in bin_chars:
        chars.extend([int(b) for b in bin])

    return np.array([[255*b] * 3 for b in chars])


def binary_to_frame(binary_stream, resolution_x, resolution_y):
    number_of_frames = len(binary_stream) / (resolution_x*resolution_y)
    
    pass

def create_discrete_frame(data:npt.NDArray[np.uint8], image):
    pass

def create_binary_frame(data:npt.NDArray[np.uint8]):
    pass

def save_frame(frame_nb: int, data: npt.NDArray[np.uint8]):
    output_frame = Image.fromarray(data)
    output_frame.save(f'frame_{frame_nb}.jpg')


if __name__ == "__main__":
    save_frame()