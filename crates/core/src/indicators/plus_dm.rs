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
    Inspired by TA-LIB PLUS_DM implementation
*/

//! Plus Directional Movement (PLUS_DM) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// PLUS_DM calculation result
/// ---
/// This struct holds the result and the state ([`PlusDmState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `plus_dm`: A vector of [`Float`] representing the calculated plus_dm.
/// - `state`: A [`PlusDmState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct PlusDmResult {
    /// The calculated PLUS_DM values
    pub plus_dm: Vec<Float>,
    /// The [`PlusDmState`] state of the PLUS_DM calculation.
    pub state: PlusDmState,
}

/// PLUS_DM calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_plus_dm`: The previous calculated value.
///
/// **State values**
/// - `prev_high`: The previous high value.
/// - `prev_low`: The previous low value.
///
/// **Parameters**
/// - `period`: The period used for the calculation.
#[derive(Debug, Clone, Copy)]
pub struct PlusDmState {
    // Outputs
    /// The previous calculated value.
    pub prev_plus_dm: Float,

    // State values
    /// The previous high value.
    pub prev_high: Float,
    /// The previous low value.
    pub prev_low: Float,

    // Parameters
    /// The period used for the calculation.
    pub period: usize,
}

/// PLUS_DM sample
/// ---
/// This struct represents a sample for the PLUS_DM calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct PlusDmSample {
    /// The current high price
    pub high: Float,
    /// The current low price
    pub low: Float,
}

impl State<&PlusDmSample> for PlusDmState {
    /// Update the [`PlusDmState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the PLUS_DM state
    fn update(&mut self, sample: &PlusDmSample) -> Result<(), TechalibError> {
        check_finite!(sample.high);
        check_finite!(sample.low);
        if self.period < 1 {
            return Err(TechalibError::BadParam(format!(
                "Period must be greater than 0, got {}",
                self.period
            )));
        }
        check_finite!(self.prev_high);
        check_finite!(self.prev_low);

        if self.period == 1 {
            let new_plus_dm =
                raw_plus_dm_unchecked(sample.high, sample.low, self.prev_high, self.prev_low);
            check_finite!(new_plus_dm);
            self.prev_plus_dm = new_plus_dm;
            self.prev_high = sample.high;
            self.prev_low = sample.low;
        } else {
            let inv_period = 1.0 / self.period as Float;
            let new_plus_dm = self.prev_plus_dm
                + plus_dm_next_unchecked(
                    sample.high,
                    sample.low,
                    self.prev_high,
                    self.prev_low,
                    self.prev_plus_dm,
                    inv_period,
                );
            check_finite!(new_plus_dm);
            self.prev_plus_dm = new_plus_dm;
            self.prev_high = sample.high;
            self.prev_low = sample.low;
        }

        Ok(())
    }
}

/// Lookback period for PLUS_DM calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    if period < 1 {
        return Err(TechalibError::BadParam(format!(
            "Period must be greater than 0, got {}",
            period
        )));
    }
    Ok(period - 1)
}

/// Calculation of the PLUS_DM function
/// ---
/// It returns a [`PlusDmResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period for the calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`PlusDmResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn plus_dm(
    high: &[Float],
    low: &[Float],
    period: usize,
) -> Result<PlusDmResult, TechalibError> {
    let mut output = vec![0.0; high.len()];

    let plus_dm_state = plus_dm_into(high, low, period, output.as_mut_slice())?;

    Ok(PlusDmResult {
        plus_dm: output,
        state: plus_dm_state,
    })
}

/// Calculation of the PLUS_DM function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`PlusDmState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period for the calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`PlusDmState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn plus_dm_into(
    high: &[Float],
    low: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<PlusDmState, TechalibError> {
    check_param_eq!(output.len(), high.len());
    check_param_eq!(output.len(), low.len());

    let len = high.len();
    let lookback = lookback_from_period(period)?;
    let inv_period = 1.0 / period as Float;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    if period == 1 {
        check_finite_at!(0, high);
        check_finite_at!(0, low);
        output[0] = Float::NAN;
        for idx in 1..len {
            check_finite_at!(idx, high);
            check_finite_at!(idx, low);
            output[idx] = raw_plus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
        }
        return Ok(PlusDmState {
            prev_plus_dm: output[len - 1],
            prev_high: high[len - 1],
            prev_low: low[len - 1],
            period,
        });
    }

    let mut plus_dm = init_plus_dm_unchecked(high, low, lookback, output)?;
    output[lookback] = plus_dm;
    check_finite_at!(lookback, output);

    for idx in lookback + 1..len {
        check_finite_at!(idx, high);
        check_finite_at!(idx, low);

        plus_dm += plus_dm_next_unchecked(
            high[idx],
            low[idx],
            high[idx - 1],
            low[idx - 1],
            plus_dm,
            inv_period,
        );

        output[idx] = plus_dm;
        check_finite_at!(idx, output);
    }

    Ok(PlusDmState {
        prev_plus_dm: output[len - 1],
        prev_high: high[len - 1],
        prev_low: low[len - 1],
        period,
    })
}

#[inline(always)]
fn init_plus_dm_unchecked(
    high: &[Float],
    low: &[Float],
    lookback: usize,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    check_finite_at!(0, high);
    check_finite_at!(0, low);
    output[0] = Float::NAN;
    let mut sum = 0.0;
    for idx in 1..lookback {
        check_finite_at!(idx, high);
        check_finite_at!(idx, low);
        sum += raw_plus_dm_unchecked(high[idx], low[idx], high[idx - 1], low[idx - 1]);
        output[idx] = Float::NAN;
    }
    check_finite_at!(lookback, high);
    check_finite_at!(lookback, low);
    sum += raw_plus_dm_unchecked(
        high[lookback],
        low[lookback],
        high[lookback - 1],
        low[lookback - 1],
    );
    Ok(sum)
}

#[inline(always)]
pub(crate) fn plus_dm_next_unchecked(
    high: Float,
    low: Float,
    prev_high: Float,
    prev_low: Float,
    prev_plus_dm: Float,
    inv_period: Float,
) -> Float {
    -prev_plus_dm * inv_period + raw_plus_dm_unchecked(high, low, prev_high, prev_low)
}

#[inline(always)]
pub(crate) fn raw_plus_dm_unchecked(
    high: Float,
    low: Float,
    prev_high: Float,
    prev_low: Float,
) -> Float {
    let plus_delta = high - prev_high;
    if plus_delta > 0.0 && (prev_low - low) < plus_delta {
        plus_delta
    } else {
        0.0
    }
}
