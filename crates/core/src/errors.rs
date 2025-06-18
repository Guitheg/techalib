/// Techalib error types
/// ---
/// This enum defines the various error types that can occur
/// during the execution of the Techalib library.
///
/// Variants
/// ---
/// - `BadParam(String)`: Indicates that a parameter passed to a function is invalid.
/// - `InsufficientData`: Indicates that there is not enough data to perform a calculation.
/// - `DataNonFinite(String)`: Indicates that a data point is not finite (e.g., NaN or Infinity).
/// - `Overflow(usize, Float)`: Indicates that an overflow occurred at a specific index.
/// - `NotImplementedYet`: Indicates that a feature or function is not yet implemented.
#[derive(Debug)]
pub enum TechalibError {
    /// Indicates that a parameter passed to a function is invalid.
    BadParam(String),
    /// Indicates that there is not enough data to perform a calculation.
    InsufficientData,
    /// Indicates that a data point is not finite (e.g., NaN or Infinity).
    DataNonFinite(String),
    /// Indicates that a feature or function is not yet implemented.
    NotImplementedYet,
}

#[macro_use]
pub(crate) mod macros {
    macro_rules! check_finite {
        ($value:expr) => {
            if !($value.is_finite()) {
                return Err(TechalibError::DataNonFinite(format!(
                    "{} = {:?}",
                    stringify!($value),
                    $value
                )));
            }
        };
        ($($value:expr),+) => {
            $(
            if !($value.is_finite()) {
                return Err(TechalibError::DataNonFinite(format!(
                "{} = {:?}",
                stringify!($value),
                $value
                )));
            }
            )+
        };
    }

    macro_rules! check_finite_at {
        ($index:expr, $data:expr) => {
            if !($data[$index].is_finite()) {
                return Err(TechalibError::DataNonFinite(format!(
                    "{}[{}] = {:?}",
                    stringify!($data),
                    $index,
                    $data[$index]
                )));
            }
        };
        ($index:expr, $($data:expr),+) => {
            $(
            if !($data[$index].is_finite()) {
            return Err(TechalibError::DataNonFinite(format!(
                "{}[{}] = {:?}",
                stringify!($data),
                $index,
                $data[$index]
            )));
            }
            )+
        };
    }

    macro_rules! check_param_eq {
        ($param:expr, $value:expr) => {
            if $param != $value {
                return Err(TechalibError::BadParam(format!(
                    "{} must be equal to {}, got {} != {}",
                    stringify!($param),
                    stringify!($value),
                    $value,
                    $param
                )));
            }
        };
    }

    macro_rules! check_param_gte {
        ($param:expr, $value:expr) => {
            if $param < $value {
                return Err(TechalibError::BadParam(format!(
                    "{} must be greater than or equal to {}, got {} < {}",
                    stringify!($param),
                    stringify!($value),
                    $value,
                    $param
                )));
            }
        };
    }

    macro_rules! check_vec_finite {
        ($vec:expr) => {
            for (i, &v) in $vec.iter().enumerate() {
                if !(v.is_finite()) {
                    return Err(TechalibError::DataNonFinite(format!(
                        "{}[{}] = {:?}",
                        stringify!($vec),
                        i,
                        v
                    )));
                }
            }
        };
    }
}
