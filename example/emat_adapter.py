import math
import ser_parser
import os
import numpy as np
import tomop
import argparse
#import matlab.engine
import scipy.signal as ss
from scipy import ndimage

parser = argparse.ArgumentParser(description='Push EMAT data to SliceRecon.')
parser.add_argument('path', metavar='path', help='path to the data')
parser.add_argument(
    '--host', default="localhost", help='the projection server host')
parser.add_argument(
    '--port', type=int, default=5558, help='the projection server port')
args = parser.parse_args()


def align(xs, ys):
    zs = ss.fftconvolve(xs, ys[::-1, ::-1])
    return np.array(np.unravel_index(np.argmax(zs, axis=None),
                                     zs.shape)) - np.array(xs.shape) + [1, 1]


# start MATLAB
#future = matlab.engine.start_matlab(async=True)
#print("Finish loading MATLAB...")
#eng = future.result()

pub = tomop.publisher(args.host, args.port)
path = args.path

count = 0
for filename in os.listdir(path):
    if filename.endswith(".ser"):
        count += 1

files = [(0, "")] * count
angles = np.zeros(count)


def to_float(str):
    return float(str[:-2])


i = 0
for filename in os.listdir(path):
    if filename.endswith(".ser"):
        full_path = os.path.join(path, filename)
        angles[i] = to_float(os.path.splitext(filename)[0])
        files[i] = (angles[i], full_path)
        i += 1

(m, n), first_proj = ser_parser.parser(files[0][1])

files = sorted(files)
angles = np.sort(angles)
print(files)

angles = angles * (math.pi / 180.0)

print(angles)

# PACKET 1: object volume specification
print("Sending object volume")
geom_spec = tomop.geometry_specification_packet(0, [-n / 2, -n / 2, -n / 2],
                                                [n / 2, n / 2, n / 2])
pub.send(geom_spec)
# PACKET 2: acquisition geometry
print("Sending acquisition geometry")
par_beam = tomop.parallel_beam_geometry_packet(0, m, n, count, angles)
pub.send(par_beam)

# PACKET 3: scan settings
print("Sending scan data")
pub.send(tomop.scan_settings_packet(0, 0, 0, True))

# PACKET 4..: Projections
prev = np.reshape(first_proj, [m, n])
prev_shift = np.round(ndimage.measurements.center_of_mass(prev)).astype(
    np.int) - np.array([m // 2 - 1, n // 2 - 1])

for idx, (angle, filename) in enumerate(files):
    print("Sending projection: ", idx)
    shape, data = ser_parser.parser(filename)
    xs = np.reshape(data, shape)
    shift = align(xs, prev)
    print(shift)
    shifted_xs = np.roll(xs, -(prev_shift + shift), (0, 1))

    pub.send(
        tomop.projection_packet(2, idx, [m, n],
                                np.ascontiguousarray(shifted_xs.ravel())))

    prev = xs
    prev_shift = prev_shift + shift
