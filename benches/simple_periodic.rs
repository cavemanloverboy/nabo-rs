use nabo_pbc::dummy_point::*;
use nabo_pbc::KDTree;
use ordered_float::NotNan;

fn main() {
    const CLOUD_SIZE: u32 = 1000000;
    let cloud = random_point_cloud(CLOUD_SIZE);
    let tree = KDTree::<_,_, 2>::new(&cloud);
    for k in [1, 2, 3, 4, 6, 8, 11, 16, 24] {
        let query = random_point();
        println!("{:?}", tree.knn_periodic(k, &query, &[NotNan::<f32>::new(100.0).unwrap(), NotNan::<f32>::new(100.0).unwrap()]));
    }
}
