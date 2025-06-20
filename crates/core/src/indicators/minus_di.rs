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
    Inspired by TA-LIB MINUS_DI implementation
*/

//! Minus Directional Indicator (MINUS_DI) implementation

use crate::errors::TechalibError;
use crate::indicators::atr::calculate_true_range;
use crate::indicators::minus_dm::{minus_dm_next_unchecked, raw_minus_dm_unchecked};
use crate::traits::State;
use crate::types::Float;

/// MINUS_DI calculation result
/// ---
/// This struct holds the result and the state ([`MinusDiState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `minus_di`: A vector of [`Float`] values representing the output
///   of the MINUS_DI calculation.
/// - `state`: A [`MinusDiState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct MinusDiResult {
    /// The output values of the MINUS_DI calculation.
    pub minus_di: Vec<Float>,
    /// The [`MinusDiState`] state of the MINUS_DI calculation.
    pub state: MinusDiState,
}

/// MINUS_DI calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_minus_di`: The previous calculated value of MINUS_DI.
///
/// **State values**
/// - `prev_minus_dm`: The previous minus directional movement (MINUS_DM).
/// - `prev_true_range`: The previous true range value.
/// - `prev_high`: The previous high price.
/// - `prev_low`: The previous low price.
/// - `prev_close`: The previous close price.
///
/// **Parameters**
/// - `period`: The period used for the MINUS_DI calculation.
#[derive(Debug, Clone, Copy)]
pub struct MinusDiState {
    // Outputs
    /// The previous calculated value of MINUS_DI.
    pub prev_minus_di: Float,
    // State values
    /// The previous minus directional movement (MINUS_DM).
    pub prev_minus_dm: Float,
    /// The previous true range value.
    pub prev_true_range: Float,
    /// The previous high price.
    pub prev_high: Float,
    /// The previous low price.
    pub prev_low: Float,
    /// The previous close price.
    pub prev_close: Float,
    // Parameters
    /// The period used for the MINUS_DI calculation.
    pub period: usize,
}

/// MINUS_DI sample
/// ---
/// This struct represents a sample for the MINUS_DI calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct MinusDiSample {
    /// The current high price
    pub high: Float,
    /// The current low price
    pub low: Float,
    /// The current close price
    pub close: Float,
}

impl State<MinusDiSample> for MinusDiState {
    /// Update the [`MinusDiState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the MINUS_DI state
    fn update(&mut self, sample: MinusDiSample) -> Result<(), TechalibError> {
        check_finite!(sample.high, sample.low, sample.close);
        check_finite!(self.prev_minus_dm);
        check_finite!(self.prev_true_range);
        check_finite!(self.prev_high, self.prev_low, self.prev_close);

        let mut new_minus_di = minus_di_next_unchecked(
            sample.high,
            sample.low,
            self.prev_high,
            self.prev_low,
            self.prev_close,
            &mut self.prev_minus_dm,
            &mut self.prev_true_range,
            1.0 / self.period as Float,
        );

        if self.period == 1 {
            new_minus_di /= 100.0;
        }

        check_finite!(&new_minus_di);
        self.prev_minus_di = new_minus_di;
        self.prev_high = sample.high;
        self.prev_low = sample.low;
        self.prev_close = sample.close;
        Ok(())
    }
}

/// Lookback period for MINUS_DI calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 1);
    Ok(period)
}

/// Calculation of the MINUS_DI function
/// ---
/// It returns a [`MinusDiResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] values representing the high prices.
/// - `low`: A slice of [`Float`] values representing the low prices.
/// - `close`: A slice of [`Float`] values representing the close prices.
/// - `period`: The period used for the MINUS_DI calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`MinusDiResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn minus_di(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
) -> Result<MinusDiResult, TechalibError> {
    let mut output_minus_di = vec![0.0; high.len()];

    let minus_di_state = minus_di_into(high, low, close, period, output_minus_di.as_mut_slice())?;

    Ok(MinusDiResult {
        minus_di: output_minus_di,
        state: minus_di_state,
    })
}

/// Calculation of the MINUS_DI function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`MinusDiState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] values representing the high prices.
/// - `low`: A slice of [`Float`] values representing the low prices.
/// - `close`: A slice of [`Float`] values representing the close prices.
/// - `period`: The period used for the MINUS_DI calculation.
///
/// Output Arguments
/// ---
/// - `output_minus_di`: A mutable slice of [`Float`] where the calculated MINUS_DI values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`MinusDiState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn minus_di_into(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
    output_minus_di: &mut [Float],
) -> Result<MinusDiState, TechalibError> {
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), close.len());
    check_param_eq!(high.len(), output_minus_di.len());
    let len = high.len();

    let lookback = lookback_from_period(period)?;
    let inv_period = 1.0 / period as Float;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    if period == 1 {
        check_finite_at!(0, high);
        check_finite_at!(0, low);
        output_minus_di[0] = Float::NAN;
        let mut minus_dm = 0.0;
        let mut true_range = 0.0;
        for idx in 1..len {
            check_finite_at!(idx, high);
            check_finite_at!(idx, low);
            minus_dm = raw_minus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
            true_range = calculate_true_range(high[idx], low[idx], close[idx - 1]);
            output_minus_di[idx] = minus_dm / true_range;
        }
        return Ok(MinusDiState {
            prev_minus_di: output_minus_di[len - 1],
            prev_minus_dm: minus_dm,
            prev_true_range: true_range,
            prev_high: high[len - 1],
            prev_low: low[len - 1],
            prev_close: close[len - 1],
            period,
        });
    }

    let (mut minus_dm, mut true_range) =
        init_minus_di_unchecked(high, low, close, lookback, output_minus_di)?;

    for idx in lookback..len {
        check_finite_at!(idx, high, low, close);

        output_minus_di[idx] = minus_di_next_unchecked(
            high[idx],
            low[idx],
            high[idx - 1],
            low[idx - 1],
            close[idx - 1],
            &mut minus_dm,
            &mut true_range,
            inv_period,
        );

        check_finite_at!(idx, output_minus_di);
    }

    Ok(MinusDiState {
        prev_minus_di: output_minus_di[len - 1],
        prev_minus_dm: minus_dm,
        prev_true_range: true_range,
        prev_high: high[len - 1],
        prev_low: low[len - 1],
        prev_close: close[len - 1],
        period,
    })
}

#[inline(always)]
fn init_minus_di_unchecked(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    lookback: usize,
    output_minus_di: &mut [Float],
) -> Result<(Float, Float), TechalibError> {
    check_finite_at!(0, high, low, close);
    output_minus_di[0] = Float::NAN;
    let mut minus_dm_sum = 0.0;
    let mut true_range_sum = 0.0;
    for idx in 1..lookback {
        check_finite_at!(idx, high, low, close);
        minus_dm_sum += raw_minus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
        true_range_sum += calculate_true_range(high[idx], low[idx], close[idx - 1]);
        output_minus_di[idx] = Float::NAN;
    }
    check_finite_at!(lookback, high, low, close);
    Ok((minus_dm_sum, true_range_sum))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn minus_di_next_unchecked(
    new_high: Float,
    new_low: Float,
    prev_high: Float,
    prev_low: Float,
    prev_close: Float,
    minus_dm: &mut Float,
    true_range: &mut Float,
    inv_period: Float,
) -> Float {
    *minus_dm += minus_dm_next_unchecked(
        new_high, new_low, prev_high, prev_low, *minus_dm, inv_period,
    );
    *true_range += -*true_range * inv_period + calculate_true_range(new_high, new_low, prev_close);
    if *true_range == 0.0 {
        return 0.0;
    }
    100.0 * *minus_dm / *true_range
}
