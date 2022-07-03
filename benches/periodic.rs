use nabo::dummy_point::*;
use nabo::KDTree;
use std::time::Instant;
use rayon::iter::*;
use ordered_float::NotNan;
use std::io::Write;
use num_traits::identities::One;

fn main() {
    
    // Number of runs
    const RUNS: u128 = 100;

    // Query and data size
    const K: usize = 3;
    const QUERY_COUNT: u32 = 1_000_000;
    const CLOUD_SIZE: u32 = 100_000;

    // Boxsize
    let periodic: [NotNan::<f64>; 3] = [NotNan::one(); 3];

    // Initialize overall millis counter
    let mut overall_build = 0;
    let mut overall_query = 0;
    let mut overall_query_periodic = 0;
    for run in 0..RUNS {

        // Generate dataset in [0, 1]^3
        let cloud = random_point_cloud_3d(CLOUD_SIZE);
        println!("building tree");
        let timer = Instant::now();
        let tree = KDTree::<_,_,K>::new_with_bucket_size(&cloud, 32);
        let build_time = timer.elapsed().as_millis();
        overall_build += build_time;

        let queries = (0..QUERY_COUNT).map(|_| random_point_p3()).collect::<Vec<_>>();

        // Save data, queries to disk
        if run == 0 {
            let filename = "test_data";
            let mut file = std::fs::File::create(filename).expect("failed to open file");
            for point in &cloud {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(24);
                
                // Write all 3 components to buffer
                buffer.append(&mut point.0[0].into_inner().to_le_bytes().to_vec());
                buffer.append(&mut point.0[1].into_inner().to_le_bytes().to_vec());
                buffer.append(&mut point.0[2].into_inner().to_le_bytes().to_vec());
                
                assert_eq!(buffer.len(), 24);
                
                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }

            let filename = "queries";
            let mut file = std::fs::File::create(filename).expect("failed to open file");
            for point in &queries {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(24);
                
                // Write all 3 components to buffer
                buffer.append(&mut point.0[0].into_inner().to_le_bytes().to_vec());
                buffer.append(&mut point.0[1].into_inner().to_le_bytes().to_vec());
                buffer.append(&mut point.0[2].into_inner().to_le_bytes().to_vec());

                assert_eq!(buffer.len(), 24);
                
                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }
        }

        // for query in &queries {
        //     tree.knn(1, query);
        // }

        println!("querying");
        let timer = Instant::now();
        let non_pbc_knns: Vec<_> = (&queries)
            .into_par_iter()
            .map_with(&tree, |t, query| {
                // println!("querying with {query:?}");
                t.knn(1, &query).pop().unwrap()
            }).collect::<Vec<_>>();
        let query_time = timer.elapsed().as_millis();
        overall_query += query_time;
        println!("building took {} millis; querying took {} millis", build_time, query_time);

        let timer = Instant::now();
        let knns: Vec<_> = queries
            .into_par_iter()
            .map_with(&tree, |t, query| {
                // println!("querying with {query:?}");
                t.knn_periodic(1, &query, &periodic).pop().unwrap()
            }).collect::<Vec<_>>();
        let query_time = timer.elapsed().as_millis();
        overall_query_periodic += query_time;
        println!("building took {} millis; periodic querying took {} millis", build_time, query_time);
        
        // Save queries to disk
        if run == 0 {
            let filename = "knns";
            let mut file = std::fs::File::create(filename).expect("failed to open file");

            for point in &non_pbc_knns {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(8);
                
                // Write distance to buffer
                buffer.append(&mut point.dist2.to_le_bytes().to_vec());

                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }

            for point in &non_pbc_knns {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(8);
                
                // Write index to buffer
                buffer.append(&mut point.index.to_le_bytes().to_vec());

                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }

            // for point in &non_pbc_knns {

            //     // Initialize byte buffer
            //     let mut buffer = Vec::with_capacity(24);
                
            //     // Write all 3 components to buffer
            //     buffer.append(&mut point.point.0[0].into_inner().to_le_bytes().to_vec());
            //     buffer.append(&mut point.point.0[1].into_inner().to_le_bytes().to_vec());
            //     buffer.append(&mut point.point.0[2].into_inner().to_le_bytes().to_vec());
                
            //     // write buffer to disk
            //     file.write_all(&mut buffer).expect("failed write to disk");
            // }

            for point in &knns {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(8);
                
                // Write distance to buffer
                buffer.append(&mut point.dist2.to_le_bytes().to_vec());

                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }

            for point in &knns {
             
                // Initialize byte buffer
                let mut buffer = Vec::with_capacity(8);
                
                // Write index to buffer
                buffer.append(&mut point.index.to_le_bytes().to_vec());

                // write buffer to disk
                file.write_all(&mut buffer).expect("failed write to disk");
            }

            // for point in &knns {

            //     // Initialize byte buffer
            //     let mut buffer = Vec::with_capacity(24);
                
            //     // Write all 3 components to buffer
            //     buffer.append(&mut point.point.0[0].into_inner().to_le_bytes().to_vec());
            //     buffer.append(&mut point.point.0[1].into_inner().to_le_bytes().to_vec());
            //     buffer.append(&mut point.point.0[2].into_inner().to_le_bytes().to_vec());
                
            //     // write buffer to disk
            //     file.write_all(&mut buffer).expect("failed write to disk");
            // }
        }
    }
    println!("average: {} millis to build, {} millis to query, {} millis to query pbc", overall_build / RUNS,  overall_query / RUNS, overall_query_periodic / RUNS);

}
