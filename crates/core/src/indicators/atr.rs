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
    Inspired by TA-LIB ATR implementation
*/

//! Average True Range (ATR) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// ATR calculation result
/// ---
/// This struct holds the result and the state ([`AtrState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated values.
/// - `state`: A [`AtrState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct AtrResult {
    /// The calculated values of the ATR.
    pub atr: Vec<Float>,
    /// The [`AtrState`] state of the ATR calculation.
    pub state: AtrState,
}

/// ATR calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `atr`: The last calculated value.
///
/// **State values**
/// - `prev_close`: The previous close price.
///
/// **Parameters**
/// - `period`: The period used for the calculation.
#[derive(Debug, Clone, Copy)]
pub struct AtrState {
    // Outputs
    /// The last calculated value.
    pub atr: Float,

    // State values
    /// The previous close price.
    pub prev_close: Float,

    // Parameters
    /// The period used for the calculation.
    pub period: usize,
}

/// ATR sample
/// ---
/// This struct represents a sample for the ATR calculation.
/// It contains the high, low and close prices.
#[derive(Debug, Clone, Copy)]
pub struct AtrSample {
    /// The high price of the sample.
    pub high: Float,
    /// The low price of the sample.
    pub low: Float,
    /// The close price of the sample.
    pub close: Float,
}

impl State<&AtrSample> for AtrState {
    /// Update the [`AtrState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the ATR state
    fn update(&mut self, sample: &AtrSample) -> Result<(), TechalibError> {
        check_finite!(sample.high);
        check_finite!(sample.low);
        check_finite!(sample.close);
        if self.period < 1 {
            return Err(TechalibError::BadParam(format!(
                "Period must be greater than 0, got {}",
                self.period
            )));
        }
        check_finite!(self.prev_close);

        let new_atr = atr_next_unchecked(
            sample.high,
            sample.low,
            self.prev_close,
            self.atr,
            1.0 / self.period as Float,
            self.period as Float - 1.0,
        );

        check_finite!(new_atr);
        self.atr = new_atr;
        self.prev_close = sample.close;

        Ok(())
    }
}

/// Lookback period for ATR calculation
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
    Ok(period)
}

/// Calculation of the ATR function
/// ---
/// It returns a [`AtrResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the closing prices.
/// - `period`: The period for the calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`AtrResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn atr(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
) -> Result<AtrResult, TechalibError> {
    let mut output = vec![0.0; close.len()];

    let atr_state = atr_into(high, low, close, period, output.as_mut_slice())?;

    Ok(AtrResult {
        atr: output,
        state: atr_state,
    })
}

/// Calculation of the ATR function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`AtrState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the closing prices.
/// - `period`: The period for the calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`AtrState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn atr_into(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<AtrState, TechalibError> {
    check_param_eq!(close.len(), high.len());
    check_param_eq!(close.len(), low.len());
    check_param_eq!(output.len(), close.len());

    let len = close.len();
    let lookback = lookback_from_period(period)?;
    let inv_period = 1.0 / period as Float;
    let period_minus_one = period as Float - 1.0;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let output_value = init_atr_unchecked(high, low, close, lookback, inv_period, output)?;
    output[lookback] = output_value;
    check_finite_at!(lookback, output);

    for idx in lookback + 1..len {
        check_finite_at!(idx - 1, close);
        check_finite_at!(idx, high);
        check_finite_at!(idx, low);

        output[idx] = atr_next_unchecked(
            high[idx],
            low[idx],
            close[idx - 1],
            output[idx - 1],
            inv_period,
            period_minus_one,
        );

        check_finite_at!(idx, output);
    }

    Ok(AtrState {
        atr: output[len - 1],
        prev_close: close[len - 1],
        period,
    })
}

#[inline(always)]
fn init_atr_unchecked(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    lookback: usize,
    inv_period: Float,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    check_finite_at!(0, close);
    output[0] = Float::NAN;
    let mut sum = 0.0;
    for idx in 1..lookback {
        check_finite_at!(idx, close);
        check_finite_at!(idx, high);
        check_finite_at!(idx, low);
        sum += calculate_true_range(high[idx], low[idx], close[idx - 1]);

        output[idx] = Float::NAN;
    }
    check_finite_at!(lookback, close);
    check_finite_at!(lookback, high);
    check_finite_at!(lookback, low);
    sum += calculate_true_range(high[lookback], low[lookback], close[lookback - 1]);

    Ok(sum * inv_period)
}

#[inline(always)]
fn atr_next_unchecked(
    high: Float,
    low: Float,
    prev_close: Float,
    prev_atr: Float,
    inv_period: Float,
    period_minus_one: Float,
) -> Float {
    (calculate_true_range(high, low, prev_close) + (prev_atr * period_minus_one)) * inv_period
}

#[inline(always)]
pub(crate) fn calculate_true_range(high: Float, low: Float, prev_close: Float) -> Float {
    let hl = high - low;
    let hc = (high - prev_close).abs();
    let lc = (low - prev_close).abs();
    if hl > hc {
        if hl > lc {
            hl
        } else {
            lc
        }
    } else if hc > lc {
        hc
    } else {
        lc
    }
}
