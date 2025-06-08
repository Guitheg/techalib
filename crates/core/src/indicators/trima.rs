use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::types::Float;

#[derive(Debug)]
pub struct TrimaResult {
    pub values: Vec<Float>,
    pub state: TrimaState,
}

#[derive(Debug, Clone)]
pub struct TrimaState {
    pub trima: Float,
    pub period: usize,
    pub period_sub: Float,
    pub period_sum: Float,
    pub window: VecDeque<Float>,
}

impl From<TrimaResult> for Vec<Float> {
    fn from(result: TrimaResult) -> Self {
        result.values
    }
}

impl TrimaState {
    pub fn next(&self, new_value: Float) -> Result<TrimaState, TechalysisError> {
        trima_next(
            new_value,
            self.trima,
            self.period,
            self.period_sub,
            self.period_sum,
            &self.window,
        )
    }
}

pub fn trima_next(
    new_value: Float,
    prev_trima: Float,
    period: usize,
    period_sub: Float,
    period_sum: Float,
    window: &VecDeque<Float>,
) -> Result<TrimaState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "TRIMA period must be greater than 1".to_string(),
        ));
    }
    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}"
        )));
    }
    if !prev_trima.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_trima = {prev_trima:?}"
        )));
    }
    if window.len() != period {
        return Err(TechalysisError::BadParam(
            "Window length must match the TRIMA period".to_string(),
        ));
    }

    for (idx, &value) in window.iter().enumerate() {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "window[{idx}] = {value:?}"
            )));
        }
    }

    let mut window = window.clone();
    let inv_weight_sum = inv_weight_sum_linear(period);

    let old_value = window
        .pop_front()
        .ok_or(TechalysisError::InsufficientData)?;
    window.push_back(new_value);

    let (trima, new_period_sub, new_period_sum) = trima_next_unchecked(
        new_value,
        old_value,
        period as Float,
        period_sub,
        period_sum,
        inv_weight_sum,
    );

    if !trima.is_finite() {
        return Err(TechalysisError::Overflow(0, trima));
    }

    Ok(TrimaState {
        trima,
        period,
        period_sub: new_period_sub,
        period_sum: new_period_sum,
        window,
    })
}

pub fn trima(data: &[Float], period: usize) -> Result<TrimaResult, TechalysisError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let trima_state = trima_into(data, period, &mut output)?;
    Ok(TrimaResult {
        values: output,
        state: trima_state,
    })
}

pub fn trima_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<TrimaState, TechalysisError> {
    let len = data.len();
    let inv_weight_sum = inv_weight_sum_linear(period);
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "TRIMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }

    let (mut period_sub, mut period_sum) =
        init_trima_unchecked(data, period, inv_weight_sum, output)?;

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        (output[idx], period_sub, period_sum) = trima_next_unchecked(
            data[idx],
            data[idx - period],
            period as Float,
            period_sub,
            period_sum,
            inv_weight_sum,
        );
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }
    Ok(TrimaState {
        trima: output[len - 1],
        period,
        period_sub,
        period_sum,
        window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

#[inline(always)]
pub fn trima_next_unchecked(
    new_value: Float,
    old_value: Float,
    period: Float,
    period_sub: Float,
    period_sum: Float,
    inv_weight_sum: Float,
) -> (Float, Float, Float) {
    let new_period_sub = period_sub - old_value + new_value;
    let new_weighted_sum = period_sum + new_value * period;
    (
        new_weighted_sum * inv_weight_sum,
        new_period_sub,
        new_weighted_sum - new_period_sub,
    )
}

#[inline(always)]
pub(crate) fn init_trima_unchecked(
    data: &[Float],
    period: usize,
    inv_weight_sum: Float,
    output: &mut [Float],
) -> Result<(Float, Float), TechalysisError> {
    let mut period_sub: Float = 0.0;
    let mut period_sum: Float = 0.0;
    for idx in 0..period {
        let weight = idx as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        period_sub += value;
        period_sum += value * weight;
        output[idx] = Float::NAN;
    }
    output[period - 1] = (period_sum + period_sub) * inv_weight_sum;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }
    Ok((period_sub, period_sum))
}

#[inline(always)]
fn inv_weight_sum_linear(period: usize) -> Float {
    2.0 / (period * (period + 1)) as Float
}
