use anyhow::Context;
use opencv::prelude::MatTraitConst;

pub fn calc_yakudo_score(image_bytes: &[u8]) -> anyhow::Result<f64> {
    let image = opencv::imgcodecs::imdecode(
        &opencv::core::Vector::<u8>::from_slice(image_bytes),
        opencv::imgcodecs::IMREAD_COLOR,
    )?;
    let mut result = opencv::core::Mat::default();
    opencv::imgproc::laplacian(
        &image,
        &mut result,
        opencv::core::CV_64F,
        1,
        1.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )
    .context("failed to calculate yakudo score")?;

    let sum = result
        .iter::<opencv::core::Point3_<f64>>()
        .unwrap()
        .map(|(_, p)| p.x + p.y + p.z)
        .sum::<f64>();
    let mean = sum / (result.rows() * result.cols() * 3) as f64;
    let variance = result
        .iter::<opencv::core::Point3_<f64>>()
        .unwrap()
        .map(|(_, p)| (p.x - mean).powi(2) + (p.y - mean).powi(2) + (p.z - mean).powi(2))
        .sum::<f64>()
        / (result.rows() * result.cols() * 3) as f64;

    let score = 1.0 / variance * 10000.0;

    Ok(score)
}
