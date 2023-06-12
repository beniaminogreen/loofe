use extendr_api::prelude::*;

use std::collections::HashMap;

#[extendr]
fn rust_loo_mean(x : &[f64]) -> Vec<f64> {
    let x_sum : f64 = x.iter().sum();
    let n = x.len() as f64;

    x.iter().map(|x| (x_sum-x)/(n-1.0)).collect::<Vec<f64>>()
}

#[extendr]
fn rust_loo_clustered_mean(x : &[f64], clust : &[i32] ) -> Vec<f64> {
    let x_sum : f64 = x.iter().sum();
    let n = x.len() as f64;

    let mut collector : HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, id) in clust.iter().enumerate() {
        if collector.contains_key(id){
            collector.get_mut(id).unwrap().push(i);
        } else {
            collector.insert(*id, vec![i as usize]);
        };
    }

    let min_length = collector.values().map(|clust| clust.len()).min().expect("could not count items!");
    if min_length == 1 {
        panic!("Mean cannot be calculated for cluster of size 1!")
    }

    let mut out_vec = vec![0.0; n as usize];

    for cluster in collector.values() {
        let cluster_sum : f64 = cluster.iter().map(|obs_id| x[*obs_id]).sum();
        let cluster_n: f64  = cluster.len() as f64;

        let cluster_loo_mean = (x_sum-cluster_sum)/(n-cluster_n as f64);

        for obs_id in cluster {
            out_vec[*obs_id] = cluster_loo_mean;
        }
    }

    out_vec

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod loofe;
    fn rust_loo_mean;
    fn rust_loo_clustered_mean;
}
