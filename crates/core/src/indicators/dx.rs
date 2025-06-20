/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

/*
    List of contributors:
    - Guitheg: Initial implementation
*/

/*
    Inspired by TA-LIB DX implementation
*/

//! Directional Index (DX) implementation

use crate::errors::TechalibError;
use crate::indicators::atr::calculate_true_range;
use crate::indicators::minus_dm::{minus_dm_next_unchecked, raw_minus_dm_unchecked};
use crate::indicators::plus_dm::{plus_dm_next_unchecked, raw_plus_dm_unchecked};
use crate::traits::State;
use crate::types::Float;

/// DX calculation result
/// ---
/// This struct holds the result and the state ([`DxState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `dx`: A vector of [`Float`] values representing the
///   Directional Index (DX) values calculated for the input data.
/// - `state`: A [`DxState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct DxResult {
    /// The DX values calculated for the input data.
    pub dx: Vec<Float>,
    /// The [`DxState`] state of the DX calculation.
    pub state: DxState,
}

/// DX calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_dx`: The last calculated DX value.
///
/// **State values**
/// - `prev_true_range`: The previous true range value.
/// - `prev_plus_dm`: The previous positive directional movement (+DM) value.
/// - `prev_minus_dm`: The previous negative directional movement (-DM) value.
/// - `prev_high`: The previous high price.
/// - `prev_low`: The previous low price.
/// - `prev_close`: The previous close price.
///
/// **Parameters**
/// - `period`: The period used for the DX calculation.
#[derive(Debug, Clone, Copy)]
pub struct DxState {
    // Outputs
    /// The last calculated DX value.
    pub prev_dx: Float,
    // State values
    /// The previous true range value.
    pub prev_true_range: Float,
    /// The previous positive directional movement (+DM) value.
    pub prev_plus_dm: Float,
    /// The previous negative directional movement (-DM) value.
    pub prev_minus_dm: Float,
    /// The previous high price.
    pub prev_high: Float,
    /// The previous low price.
    pub prev_low: Float,
    /// The previous close price.
    pub prev_close: Float,
    // Parameters
    /// The period used for the DX calculation.
    pub period: usize,
}

/// DX sample
/// ---
/// This struct represents a sample for the DX calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct DxSample {
    /// The high price of the sample.
    pub high: Float,
    /// The low price of the sample.
    pub low: Float,
    /// The close price of the sample.
    pub close: Float,
}

impl State<DxSample> for DxState {
    /// Update the [`DxState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the DX state
    fn update(&mut self, sample: DxSample) -> Result<(), TechalibError> {
        check_finite!(sample.high, sample.low, sample.close);
        check_finite!(self.prev_true_range);
        check_finite!(self.prev_plus_dm);
        check_finite!(self.prev_minus_dm);
        check_finite!(self.prev_high);
        check_finite!(self.prev_low);
        check_finite!(self.prev_close);

        let new_dx = dx_next_unchecked(
            sample.high,
            sample.low,
            &mut self.prev_true_range,
            &mut self.prev_plus_dm,
            &mut self.prev_minus_dm,
            self.prev_high,
            self.prev_low,
            self.prev_close,
            1.0 / self.period as Float,
        );

        check_finite!(&new_dx);
        self.prev_dx = new_dx;
        self.prev_high = sample.high;
        self.prev_low = sample.low;
        self.prev_close = sample.close;
        Ok(())
    }
}

/// Lookback period for DX calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 2);
    Ok(period)
}

/// Calculation of the DX function
/// ---
/// It returns a [`DxResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of high prices.
/// - `low`: A slice of low prices.
/// - `close`: A slice of close prices.
/// - `period`: The period used for the DX calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`DxResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn dx(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
) -> Result<DxResult, TechalibError> {
    let mut output_dx = vec![0.0; high.len()];

    let dx_state = dx_into(high, low, close, period, output_dx.as_mut_slice())?;

    Ok(DxResult {
        dx: output_dx,
        state: dx_state,
    })
}

/// Calculation of the DX function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`DxState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of high prices.
/// - `low`: A slice of low prices.
/// - `close`: A slice of close prices.
///
/// Output Arguments
/// ---
/// - `period`: The period used for the DX calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`DxState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn dx_into(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
    output_dx: &mut [Float],
) -> Result<DxState, TechalibError> {
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), close.len());
    check_param_eq!(high.len(), output_dx.len());
    let len = high.len();

    let lookback = lookback_from_period(period)?;
    let inv_period = 1.0 / period as Float;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let (mut true_range, mut plus_dm, mut minus_dm) =
        init_dx_unchecked(high, low, close, lookback, output_dx)?;

    for idx in lookback..len {
        check_finite_at!(idx, high, low, close);

        output_dx[idx] = dx_next_unchecked(
            high[idx],
            low[idx],
            &mut true_range,
            &mut plus_dm,
            &mut minus_dm,
            high[idx - 1],
            low[idx - 1],
            close[idx - 1],
            inv_period,
        );

        check_finite_at!(idx, output_dx);
    }

    Ok(DxState {
        prev_dx: output_dx[len - 1],
        prev_true_range: true_range,
        prev_plus_dm: plus_dm,
        prev_minus_dm: minus_dm,
        prev_high: high[len - 1],
        prev_low: low[len - 1],
        prev_close: close[len - 1],
        period,
    })
}

#[inline(always)]
pub(crate) fn init_dx_unchecked(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    lookback: usize,
    output_dx: &mut [Float],
) -> Result<(Float, Float, Float), TechalibError> {
    check_finite_at!(0, high, low, close);
    output_dx[0] = Float::NAN;
    let mut minus_dm_sum = 0.0;
    let mut plus_dm_sum = 0.0;
    let mut true_range_sum = 0.0;
    for idx in 1..lookback {
        check_finite_at!(idx, high, low, close);
        minus_dm_sum += raw_minus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
        plus_dm_sum += raw_plus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
        true_range_sum += calculate_true_range(high[idx], low[idx], close[idx - 1]);
        output_dx[idx] = Float::NAN;
    }
    check_finite_at!(lookback, high, low, close);
    Ok((true_range_sum, plus_dm_sum, minus_dm_sum))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub(crate) fn dx_next_unchecked(
    new_high: Float,
    new_low: Float,
    true_range: &mut Float,
    plus_dm: &mut Float,
    minus_dm: &mut Float,
    prev_high: Float,
    prev_low: Float,
    prev_close: Float,
    inv_period: Float,
) -> Float {
    *plus_dm +=
        plus_dm_next_unchecked(new_high, new_low, prev_high, prev_low, *plus_dm, inv_period);
    *minus_dm += minus_dm_next_unchecked(
        new_high, new_low, prev_high, prev_low, *minus_dm, inv_period,
    );
    *true_range += -*true_range * inv_period + calculate_true_range(new_high, new_low, prev_close);
    if *true_range == 0.0 {
        return 0.0;
    }
    calculate_dx(*plus_dm / *true_range, *minus_dm / *true_range)
}

#[inline(always)]
fn calculate_dx(plus_di: Float, minus_di: Float) -> Float {
    if plus_di + minus_di == 0.0 {
        return 0.0;
    }
    ((plus_di - minus_di).abs() / (plus_di + minus_di)) * 100.0
}
