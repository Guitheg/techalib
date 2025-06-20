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
    Inspired by TA-LIB AD implementation
*/

//! Chaikin Accumulation/Distribution Line (AD) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// AD calculation result
/// ---
/// This struct holds the result and the state ([`AdState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated values.
/// - `state`: A [`AdState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct AdResult {
    /// The calculated values of the AD function.
    pub ad: Vec<Float>,
    /// The [`AdState`] state of the AD calculation.
    pub state: AdState,
}

/// AD calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `ad`: The last calculated value.
#[derive(Debug, Clone, Copy)]
pub struct AdState {
    // Outputs
    /// The last calculated value.
    pub ad: Float,
}

/// AD sample
/// ---
/// This struct represents a sample for the AD calculation.
/// It contains the high, low, close prices and volume.
#[derive(Debug, Clone, Copy)]
pub struct AdSample {
    /// The high price of the sample.
    pub high: Float,
    /// The low price of the sample.
    pub low: Float,
    /// The close price of the sample.
    pub close: Float,
    /// The volume of the sample.
    pub volume: Float,
}

impl State<&AdSample> for AdState {
    /// Update the [`AdState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the AD state
    fn update(&mut self, sample: &AdSample) -> Result<(), TechalibError> {
        check_finite!(sample.high);
        check_finite!(sample.low);
        check_finite!(sample.close);
        check_finite!(sample.volume);

        let new_ad = ad_next_unchecked(
            sample.high,
            sample.low,
            sample.close,
            sample.volume,
            self.ad,
        );

        check_finite!(new_ad);

        self.ad = new_ad;

        Ok(())
    }
}

/// Calculation of the AD function
/// ---
/// It returns a [`AdResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the close prices.
/// - `volume`: A slice of [`Float`] representing the volume.
///
/// Returns
/// ---
/// A `Result` containing a [`AdResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn ad(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    volume: &[Float],
) -> Result<AdResult, TechalibError> {
    let mut output = vec![0.0; high.len()];

    let ad_state = ad_into(high, low, close, volume, output.as_mut_slice())?;

    Ok(AdResult {
        ad: output,
        state: ad_state,
    })
}

/// Calculation of the AD function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`AdState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `close`: A slice of [`Float`] representing the close prices.
/// - `volume`: A slice of [`Float`] representing the volume.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`AdState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn ad_into(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    volume: &[Float],
    output: &mut [Float],
) -> Result<AdState, TechalibError> {
    check_param_eq!(output.len(), high.len());
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), close.len());
    check_param_eq!(high.len(), volume.len());

    let len = high.len();

    if len == 0 {
        return Err(TechalibError::InsufficientData);
    }

    let mut ad = init_ad_unchecked(high, low, close, volume)?;
    output[0] = ad;
    check_finite_at!(0, output);

    for idx in 1..len {
        check_finite_at!(idx, high);
        check_finite_at!(idx, low);
        check_finite_at!(idx, close);
        check_finite_at!(idx, volume);

        ad += money_flow_multiplier_unchecked(high[idx], low[idx], close[idx]) * volume[idx];

        output[idx] = ad;

        check_finite_at!(idx, output);
    }

    Ok(AdState {
        ad: output[len - 1],
    })
}

#[inline(always)]
fn init_ad_unchecked(
    high: &[Float],
    low: &[Float],
    close: &[Float],
    volume: &[Float],
) -> Result<Float, TechalibError> {
    check_finite_at!(0, high);
    check_finite_at!(0, low);
    check_finite_at!(0, close);
    check_finite_at!(0, volume);
    Ok(money_flow_multiplier_unchecked(high[0], low[0], close[0]) * volume[0])
}

#[inline(always)]
fn ad_next_unchecked(
    high: Float,
    low: Float,
    close: Float,
    volume: Float,
    last_ad: Float,
) -> Float {
    last_ad + money_flow_multiplier_unchecked(high, low, close) * volume
}

#[inline(always)]
fn money_flow_multiplier_unchecked(high: Float, low: Float, close: Float) -> Float {
    if high <= low {
        return 0.0;
    }
    (close - low - (high - close)) / (high - low)
}
