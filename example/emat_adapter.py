import math
import ser_parser
import os
import numpy as np
import tomop
import argparse

parser = argparse.ArgumentParser(
    description='Push EMAT data to SliceRecon.')
parser.add_argument('path', metavar='path', help='path to the data')
parser.add_argument(
    '--host', default="localhost", help='the projection server host')
parser.add_argument(
    '--port', type=int, default=5558, help='the projection server port')
args = parser.parse_args()

pub = tomop.publisher(args.host, args.port)
path = args.path

count = 0
for filename in os.listdir(path):
    if filename.endswith(".ser"):
        count += 1

files = [(0, "")] * count
angles = np.zeros(count)


def to_int(str):
    return int(str[:-2])


i = 0
for filename in os.listdir(path):
    if filename.endswith(".ser"):
        full_path = os.path.join(path, filename)
        angles[i] = to_int(os.path.splitext(filename)[0])
        files[i] = (angles[i], full_path)
        i += 1

(m, n), _ = ser_parser.parser(files[0][1])

angles = angles * (math.pi / 180.0)

# PACKET 1: object volume specification
print("Sending object volume")
geom_spec = tomop.geometry_specification_packet(0, [-n / 2, -n / 2, -n / 2],
                                    [n / 2, n / 2, n / 2])
pub.send(geom_spec)
# PACKET 2: acquisition geometry
print("Sending acquisition geometry")
par_beam = tomop.parallel_beam_geometry_packet(0, m, n, count, angles)
pub.send(par_beam)


# PACKET 3..: Projections
# A) send (fake) dark
print("Sending dark")
fake_data = np.ascontiguousarray(np.zeros([m, n]).flatten())
pub.send(tomop.projection_packet(0, 0, [m, n], fake_data))
# B) send (fake) flat
print("Sending flat")
fake_flat_data = np.ascontiguousarray(np.ones([m, n]).flatten())
pub.send(tomop.projection_packet(1, 0, [m, n], fake_flat_data))
# C) send projections
# FIXME dont neg log?
for idx, (angle, filename) in enumerate(files):
    print("Sending projection: ", idx)
    shape, data = ser_parser.parser(filename)
    pub.send(tomop.projection_packet(2, idx, [m, n], data))
