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
    Inspired by TA-LIB ADX implementation
*/

//! Average Directional Index (ADX) implementation

use crate::errors::TechalibError;
use crate::indicators::dx::{dx_next_unchecked, init_dx_unchecked};
use crate::traits::State;
use crate::types::Float;

/// ADX calculation result
/// ---
/// This struct holds the result and the state ([`AdxState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `adx`: A vector of [`Float`] representing the calculated ADX values.
/// - `state`: A [`AdxState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct AdxResult {
    /// The calculated ADX values.
    pub adx: Vec<Float>,
    /// The [`AdxState`] state of the ADX calculation.
    pub state: AdxState,
}

/// ADX calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_adx`: The last calculated ADX value.
///
/// **State values**
/// - `prev_true_range`: The previous true range value.
/// - `prev_plus_dm`: The previous positive directional movement value.
/// - `prev_minus_dm`: The previous negative directional movement value.
/// - `prev_high`: The previous high price.
/// - `prev_low`: The previous low price.
/// - `prev_close`: The previous close price.
///
/// **Parameters**
/// - `period`: The period used for the ADX calculation, which determines
///   how many values are averaged to compute the ADX.
#[derive(Debug, Clone)]
pub struct AdxState {
    // Outputs
    /// The last calculated ADX value.
    pub prev_adx: Float,
    // State values
    /// The previous true range value.
    pub prev_true_range: Float,
    /// The previous positive directional movement value.
    pub prev_plus_dm: Float,
    /// The previous negative directional movement value.
    pub prev_minus_dm: Float,
    /// The previous high price.
    pub prev_high: Float,
    /// The previous low price.
    pub prev_low: Float,
    /// The previous close price.
    pub prev_close: Float,
    // Parameters
    /// The period used for the ADX calculation.
    pub period: usize,
}

/// ADX sample
/// ---
/// This struct represents a sample for the ADX calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct AdxSample {
    /// The high price of the sample.
    pub high: Float,
    /// The low price of the sample.
    pub low: Float,
    /// The close price of the sample.
    pub close: Float,
}

impl State<&AdxSample> for AdxState {
    /// Update the [`AdxState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the ADX state
    fn update(&mut self, sample: &AdxSample) -> Result<(), TechalibError> {
        check_finite!(sample.high, sample.low, sample.close);
        check_finite!(self.prev_true_range);
        check_finite!(self.prev_plus_dm);
        check_finite!(self.prev_minus_dm);
        check_finite!(self.prev_high);
        check_finite!(self.prev_low);
        check_finite!(self.prev_close);

        let new_adx = adx_next_unchecked(
            self.prev_adx,
            sample.high,
            sample.low,
            &mut self.prev_true_range,
            &mut self.prev_plus_dm,
            &mut self.prev_minus_dm,
            self.prev_high,
            self.prev_low,
            self.prev_close,
            1.0 / self.period as Float,
            self.period as Float - 1.0,
        );

        check_finite!(&new_adx);
        self.prev_adx = new_adx;
        self.prev_high = sample.high;
        self.prev_low = sample.low;
        self.prev_close = sample.close;

        Ok(())
    }
}

/// Lookback period for ADX calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 2);
    Ok((period * 2) - 1)
}

/// Calculation of the ADX function
/// ---
/// It returns a [`AdxResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the close prices.
/// - `period`: The period for the ADX calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`AdxResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn adx(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
) -> Result<AdxResult, TechalibError> {
    let mut output_adx = vec![0.0; high.len()];

    let adx_state = adx_into(high, low, close, period, output_adx.as_mut_slice())?;

    Ok(AdxResult {
        adx: output_adx,
        state: adx_state,
    })
}

/// Calculation of the ADX function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`AdxState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the close prices.
/// - `period`: The period for the ADX calculation.
///
/// Output Arguments
/// ---
/// - `output_adx`: A mutable slice of [`Float`] where the calculated ADX values
///
/// Returns
/// ---
/// A `Result` containing a [`AdxState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn adx_into(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
    output_adx: &mut [Float],
) -> Result<AdxState, TechalibError> {
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), close.len());
    check_param_eq!(high.len(), output_adx.len());
    let len = high.len();

    let lookback = lookback_from_period(period)?;
    let inv_period = 1.0 / (period as Float);
    let period_minus_one = period as Float - 1.0;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let (first_output_adx, mut true_range, mut plus_dm, mut minus_dm) =
        init_adx_unchecked(high, low, close, period, lookback, inv_period, output_adx)?;
    output_adx[lookback] = first_output_adx;
    check_finite_at!(lookback, output_adx);

    for idx in lookback + 1..len {
        check_finite_at!(idx, high, low, close);

        output_adx[idx] = adx_next_unchecked(
            output_adx[idx - 1],
            high[idx],
            low[idx],
            &mut true_range,
            &mut plus_dm,
            &mut minus_dm,
            high[idx - 1],
            low[idx - 1],
            close[idx - 1],
            inv_period,
            period_minus_one,
        );

        check_finite_at!(idx, output_adx);
    }

    Ok(AdxState {
        prev_adx: output_adx[len - 1],
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
fn init_adx_unchecked(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
    lookback: usize,
    inv_period: Float,
    output_adx: &mut [Float],
) -> Result<(Float, Float, Float, Float), TechalibError> {
    let (mut true_range, mut plus_dm, mut minus_dm) =
        init_dx_unchecked(high, low, close, period, output_adx)?;
    let mut last_dx = 0.0;
    let mut dx_sum = last_dx;
    for idx in period..=lookback {
        check_finite_at!(idx, high, low, close);
        last_dx = dx_next_unchecked(
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
        dx_sum += last_dx;
        output_adx[idx] = Float::NAN;
    }
    Ok((dx_sum * inv_period, true_range, plus_dm, minus_dm))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn adx_next_unchecked(
    prev_value: Float,
    new_high: Float,
    new_low: Float,
    true_range: &mut Float,
    plus_dm: &mut Float,
    minus_dm: &mut Float,
    prev_high: Float,
    prev_low: Float,
    prev_close: Float,
    inv_period: Float,
    period_minus_one: Float,
) -> Float {
    let new_dx = dx_next_unchecked(
        new_high, new_low, true_range, plus_dm, minus_dm, prev_high, prev_low, prev_close,
        inv_period,
    );

    (prev_value * period_minus_one + new_dx) * inv_period
}
