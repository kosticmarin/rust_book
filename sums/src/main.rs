fn main() {
    let v = vec![vec![1., 2., 3.], vec![4., 3., 2.], vec![5., 5., 5.]];
    dbg!(&v);
    let avg: Vec<f32> = v
        .iter()
        .fold(vec![0.; v.len()], |sum, x| {
            sum.iter().zip(x.iter()).map(|(v1, v2)| *v1 + *v2).collect()
        })
        .iter()
        .map(|x| *x / v.len() as f32)
        .collect();
    dbg!(&avg);
}
