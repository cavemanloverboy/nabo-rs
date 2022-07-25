use nabo_pbc::dummy_point::*;
use nabo_pbc::KDTree;
use nabo_pbc::Point;

fn main() {
    const QUERY_COUNT: u32 = 20000;
    const CLOUD_SIZE: u32 = 1000000;
    let cloud = random_point_cloud(CLOUD_SIZE);
    let tree = KDTree::<_,_,{ P2::DIM as usize }>::new(&cloud);
    let queries = (0..QUERY_COUNT).map(|_| random_point()).collect::<Vec<_>>();
    for k in [1, 2, 3, 4, 6, 8, 11, 16, 24] {
        for query in &queries {
            tree.knn(k, query);
        }
    }
}
