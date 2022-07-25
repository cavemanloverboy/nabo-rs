from scipy.spatial import cKDTree as Tree
import numpy as np
from time import time

Nruns = 10


building = 0
querying = 0
OPTIONAL_FEATURES = False

Nd = 10**5
Nq = 10**6
K = 4

K = [1] if K == 1 else K

for _ in range(Nruns):

    data = np.random.uniform(size=(Nd, 3)).astype(np.float64)
    query = np.random.uniform(size=(Nq, 3)).astype(np.float64)

    start = time()
    tree = Tree(data, compact_nodes=OPTIONAL_FEATURES, balanced_tree=OPTIONAL_FEATURES, leafsize=32)
    building += time() - start

    start = time()
    r, ids = tree.query(query, k=K, workers=48)
    querying += time() - start

avg_query_time = querying / Nruns * 1000
avg_build_time = building / Nruns * 1000
print(f"finished non-pbc with {avg_query_time} millis average querying, {avg_build_time} building")


building = 0
querying = 0
for _ in range(Nruns):

    data = np.random.uniform(size=(Nd, 3)).astype(np.float64)
    query = np.random.uniform(size=(Nq, 3)).astype(np.float64)

    start = time()
    tree = Tree(data, compact_nodes=OPTIONAL_FEATURES, balanced_tree=OPTIONAL_FEATURES, leafsize=32, boxsize=[1.0, 1.0, 1.0])
    building += time() - start

    start = time()
    r, ids = tree.query(query, k=K, workers=48)
    querying += time() - start

avg_query_time = querying / Nruns * 1000
avg_build_time = building / Nruns * 1000
print(f"finished     pbc with {avg_query_time} millis average querying, {avg_build_time} building")



QUERY_COUNT = 100
QUERY_COUNT = 10**6
CLOUD_SIZE = 10
CLOUD_SIZE = 10**5

query_file = open("queries", "rb")
query = np.frombuffer(query_file.read(), dtype=np.float64).reshape(QUERY_COUNT, 3)
query_file.close()

data_file = open("test_data", "rb")
data = data_file.read()
data = np.frombuffer(data, dtype=np.float64).reshape(CLOUD_SIZE, 3)
data_file.close()

knn_file = open("knns", "rb")
non_pbc_r = np.frombuffer(knn_file.read(8*QUERY_COUNT*K), dtype=np.float64).reshape(QUERY_COUNT, K)
non_pbc_ids = np.frombuffer(knn_file.read(4*QUERY_COUNT*K), dtype=np.int32).reshape(QUERY_COUNT, K)
# non_pbc_r_query = np.frombuffer(knn_file.read(3*8*QUERY_COUNT), dtype=np.float64).reshape(QUERY_COUNT, 3)
r = np.frombuffer(knn_file.read(8*QUERY_COUNT*K), dtype=np.float64).reshape(QUERY_COUNT, K)
ids = np.frombuffer(knn_file.read(4*QUERY_COUNT*K), dtype=np.int32).reshape(QUERY_COUNT, K)
# r_query = np.frombuffer(knn_file.read(3*8*QUERY_COUNT), dtype=np.float64).reshape(QUERY_COUNT, 3)

tree = Tree(data, leafsize=32)
non_pbc_r2, non_pbc_ids2 = tree.query(query, k=K, eps=0)

tree = Tree(data, leafsize=32, boxsize=1.0)
r2, ids2 = tree.query(query, k=K, eps=0)

# print(data)
# print(query)
# print(r, ids)
print("num matches    :", np.isclose(r, r2**2).sum())
print("num matches pbc:", np.isclose(non_pbc_r, non_pbc_r2**2).sum())