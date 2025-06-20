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
    Inspired by TA-LIB MIDPRICE implementation
*/

//! Middle price (MIDPRICE) implementation

use std::collections::VecDeque;

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// MIDPRICE calculation result
/// ---
/// This struct holds the result and the state ([`MidpriceState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `midprice`: A vector of [`Float`] representing the calculated MIDPRICE values.
/// - `state`: A [`MidpriceState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct MidpriceResult {
    /// The calculated MIDPRICE values.
    pub midprice: Vec<Float>,
    /// The [`MidpriceState`] state of the MIDPRICE calculation.
    pub state: MidpriceState,
}

/// MIDPRICE calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `midprice`: The last calculated MIDPRICE value.
///
/// **State values**
/// - `last_high_window`: A deque containing the last `period` high prices used for the MIDPRICE calculation.
/// - `last_low_window`: A deque containing the last `period` low prices used for the MIDPRICE calculation.
///
/// **Parameters**
/// - `period`: The period used for the MIDPRICE calculation.
#[derive(Debug, Clone)]
pub struct MidpriceState {
    // Outputs
    /// The last calculated MIDPRICE value.
    pub midprice: Float,

    // State values
    /// A deque containing the last `period` high prices used for the MIDPRICE calculation.
    pub last_high_window: VecDeque<Float>,
    /// A deque containing the last `period` low prices used for the MIDPRICE calculation.
    pub last_low_window: VecDeque<Float>,

    // Parameters
    /// The period used for the MIDPRICE calculation.
    pub period: usize,
}

/// MIDPRICE sample
/// ---
/// This struct represents a sample for the MIDPRICE calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct MidpriceSample {
    /// The high price of the sample
    pub high: Float,
    /// The low price of the sample
    pub low: Float,
}

impl State<&MidpriceSample> for MidpriceState {
    /// Update the [`MidpriceState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input [`MidpriceSample`] to update the MIDPRICE state.
    ///   It contains the high and low prices of the sample.
    fn update(&mut self, sample: &MidpriceSample) -> Result<(), TechalibError> {
        check_finite!(sample.high);
        check_finite!(sample.low);

        if self.period <= 1 {
            return Err(TechalibError::BadParam(format!(
                "Period must be greater than 1, got: {}",
                self.period
            )));
        }

        if self.last_high_window.len() != self.period {
            return Err(TechalibError::BadParam(format!(
                "MIDPRICE state last_high_window length must be equal to period ({}), got: {}",
                self.period,
                self.last_high_window.len()
            )));
        }

        if self.last_low_window.len() != self.period {
            return Err(TechalibError::BadParam(format!(
                "MIDPRICE state last_low_window length must be equal to period ({}), got: {}",
                self.period,
                self.last_low_window.len()
            )));
        }

        for (idx, &value) in self.last_high_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalibError::DataNonFinite(format!(
                    "last_high_window[{idx}] = {value:?}"
                )));
            }
        }

        for (idx, &value) in self.last_low_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalibError::DataNonFinite(format!(
                    "last_low_window[{idx}] = {value:?}"
                )));
            }
        }

        let mut high_window = self.last_high_window.clone();
        let mut low_window = self.last_low_window.clone();

        let _ = high_window
            .pop_front()
            .ok_or(TechalibError::InsufficientData)?;
        high_window.push_back(sample.high);

        let _ = low_window
            .pop_front()
            .ok_or(TechalibError::InsufficientData)?;
        low_window.push_back(sample.low);

        let mid_price =
            midprice_next_unchecked(high_window.make_contiguous(), low_window.make_contiguous());
        check_finite!(mid_price);
        self.last_high_window = high_window;
        self.last_low_window = low_window;
        self.midprice = mid_price;

        Ok(())
    }
}

/// Lookback period for MIDPRICE calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    if period <= 1 {
        return Err(TechalibError::BadParam(format!(
            "Period must be greater than 1, got: {}",
            period
        )));
    }
    Ok(period - 1)
}

/// Calculation of the MIDPRICE function
/// ---
/// It returns a [`MidpriceResult`]
///
/// Input Arguments
/// ---
/// - `high_prices`: A slice of [`Float`] representing the high prices.
/// - `low_prices`: A slice of [`Float`] representing the low prices.
/// - `period`: The period for the calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`MidpriceResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn midprice(
    high_prices: &[Float],
    low_prices: &[Float],
    period: usize,
) -> Result<MidpriceResult, TechalibError> {
    let mut output = vec![0.0; high_prices.len()];

    let midprice_state = midprice_into(high_prices, low_prices, period, output.as_mut_slice())?;

    Ok(MidpriceResult {
        midprice: output,
        state: midprice_state,
    })
}

/// Calculation of the MIDPRICE function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`MidpriceState`].
///
/// Input Arguments
/// ---
/// - `high_prices`: A slice of [`Float`] representing the high prices.
/// - `low_prices`: A slice of [`Float`] representing the low prices.
/// - `period`: The period for the calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`MidpriceState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn midprice_into(
    high_prices: &[Float],
    low_prices: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<MidpriceState, TechalibError> {
    check_param_eq!(high_prices.len(), low_prices.len());
    check_param_eq!(output.len(), high_prices.len());

    let len = high_prices.len();
    let lookback = lookback_from_period(period)?;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let midprice = init_midprice_unchecked(high_prices, low_prices, lookback, output)?;
    check_finite!(midprice);
    output[lookback] = midprice;

    for idx in lookback + 1..len {
        check_finite_at!(idx, high_prices);
        check_finite_at!(idx, low_prices);
        output[idx] = midprice_next_unchecked(
            &high_prices[idx - lookback..=idx],
            &low_prices[idx - lookback..=idx],
        );
        check_finite!(output[idx]);
    }

    Ok(MidpriceState {
        midprice: output[len - 1],
        last_high_window: VecDeque::from(high_prices[len - period..len].to_vec()),
        last_low_window: VecDeque::from(low_prices[len - period..len].to_vec()),
        period,
    })
}

#[inline(always)]
fn init_midprice_unchecked(
    high_prices: &[Float],
    low_prices: &[Float],
    lookback: usize,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    check_finite_at!(0, high_prices);
    check_finite_at!(0, low_prices);
    let mut maximum = high_prices[0];
    let mut minimum = low_prices[0];
    output[0] = f64::NAN;
    for i in 0..lookback {
        check_finite_at!(i, high_prices);
        check_finite_at!(i, low_prices);
        (maximum, minimum) = maxmin(high_prices[i], low_prices[i], maximum, minimum);
        output[i] = f64::NAN;
    }
    check_finite_at!(lookback, high_prices);
    check_finite_at!(lookback, low_prices);
    (maximum, minimum) = maxmin(
        high_prices[lookback],
        low_prices[lookback],
        maximum,
        minimum,
    );
    Ok(calculate_midprice(maximum, minimum))
}

#[inline(always)]
fn midprice_next_unchecked(last_high_window: &[Float], last_low_window: &[Float]) -> Float {
    let mut maximum = last_high_window[0];
    let mut minimum = last_low_window[0];
    for j in 0..last_high_window.len() {
        (maximum, minimum) = maxmin(last_high_window[j], last_low_window[j], maximum, minimum);
    }
    calculate_midprice(maximum, minimum)
}

#[inline(always)]
pub(crate) fn maxmin(
    high_value: Float,
    low_value: Float,
    maximum: Float,
    minimum: Float,
) -> (Float, Float) {
    (
        if high_value > maximum {
            high_value
        } else {
            maximum
        },
        if low_value < minimum {
            low_value
        } else {
            minimum
        },
    )
}

const HALF: Float = 0.5;

#[inline(always)]
fn calculate_midprice(maximum: Float, minimum: Float) -> Float {
    (maximum + minimum) * HALF
}
