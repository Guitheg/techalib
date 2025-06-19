import csv
from typing import List
import numpy as np
import pandas as pd
import talib
from pathlib import Path
import argparse
from utils.logger import logger
from utils import ohlcv
from utils.kwargs_parser import ParseKwargs

DATA_DIR = Path(__file__).parent.parent / "tests" / "data" / "generated"
DATA_DIR.mkdir(parents=True, exist_ok=True)
RAND = np.random.default_rng(seed=42)

class Configuration():
    def __init__(self, module: object, fct_name: str, input_names: List[str], parameters: dict, output_names: List[str], sample_size: int = 1000):
        self.module = module
        self.fct_name = fct_name
        self.input_names = input_names
        self.parameters = parameters
        self.output_names = output_names
        self.sample_size = sample_size

CONFIG_DICT = {
    "EMA": Configuration(talib, "EMA", ["close"], dict(timeperiod=30), ["ema"]),
    "SMA": Configuration(talib, "SMA", ["close"], dict(timeperiod=30), ["sma"]),
    "RSI": Configuration(talib, "RSI", ["close"], dict(timeperiod=14), ["rsi"]),
    "MACD": Configuration(talib, "MACD", ["close"], dict(fastperiod=12, slowperiod=26, signalperiod=9), ["macd", "signal", "histogram"]),
    "BBANDS": Configuration(talib, "BBANDS", ["close"], dict(timeperiod=20, nbdevup=2, nbdevdn=2, matype=0), ["upper", "middle", "lower"]),
    "WMA": Configuration(talib, "WMA", ["close"], dict(timeperiod=30), ["wma"]),
    "DEMA": Configuration(talib, "DEMA", ["close"], dict(timeperiod=30), ["dema"]),
    "TEMA": Configuration(talib, "TEMA", ["close"], dict(timeperiod=30), ["tema"]),
    "TRIMA": Configuration(talib, "TRIMA", ["close"], dict(timeperiod=30), ["trima"]),
    "T3": Configuration(talib, "T3", ["close"], dict(timeperiod=20, vfactor=0.7), ["t3"]),
    "KAMA": Configuration(talib, "KAMA", ["close"], dict(timeperiod=30), ["kama"]),
    "MIDPOINT": Configuration(talib, "MIDPOINT", ["close"], dict(timeperiod=14), ["midpoint"]),
    "MIDPRICE": Configuration(talib, "MIDPRICE", ["high", "low"], dict(timeperiod=14), ["midprice"]),
    "ROC": Configuration(talib, "ROC", ["close"], dict(timeperiod=10), ["roc"]),
    "ATR": Configuration(talib, "ATR", ["high", "low", "close"], dict(timeperiod=14), ["atr"]),
    "AD": Configuration(talib, "AD", ["high", "low", "close", "volume"], dict(), ["ad"]),
    "MINUS_DM": Configuration(talib, "MINUS_DM", ["high", "low"], dict(timeperiod=14), ["minus_dm"]),
    "PLUS_DM": Configuration(talib, "PLUS_DM", ["high", "low"], dict(timeperiod=14), ["plus_dm"]),
    "MINUS_DI": Configuration(talib, "MINUS_DI", ["high", "low", "close"], dict(timeperiod=14), ["minus_di"]),
    "PLUS_DI": Configuration(talib, "PLUS_DI", ["high", "low", "close"], dict(timeperiod=14), ["plus_di"]),
    "DX": Configuration(talib, "DX", ["high", "low", "close"], dict(timeperiod=14), ["dx"]),
}

def generate_test_data(filename: str, configuration: Configuration, seed: int):
    logger.info(f"📊 ({configuration.fct_name}) Generating test data with parameters: {configuration.parameters}")
    generated_data = ohlcv.random_walk(configuration.sample_size, scale=1.5, start_offset = 50, seed = seed)
    output_data = getattr(configuration.module, configuration.fct_name).__call__(
        *[generated_data[name].values for name in configuration.input_names],
        **configuration.parameters
    )
    if isinstance(output_data, tuple):
        output_df = pd.DataFrame(
            {
                name: output_data[i] for i, name in enumerate(configuration.output_names)
            }
        )
    else:
        output_df = pd.DataFrame(
            {
                configuration.output_names[0]: output_data
            }
        )

    final_df = pd.concat([generated_data[configuration.input_names], output_df], axis=1)

    final_df.to_csv(
        DATA_DIR / f"{filename}.csv",
        index=False,
        float_format="%.8f",
        na_rep="nan",
    )

    logger.info(f"✅ ({configuration.fct_name}) Successfully write data at : {DATA_DIR / filename}.csv")

def dict_to_posix_filename(d: dict) -> str:
    """Convert a dictionary to a posix filename."""
    return "_".join(f"{k}={v}" for k, v in d.items() if v is not None).replace(" ", "_").replace("/", "_").replace("\\", "_").replace("=","-")

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", "--name", type=str)
    parser.add_argument("--seed", type=int, default=5)
    parser.add_argument("--args", nargs='*', action=ParseKwargs)
    parser.add_argument("--size", type=int, default=1000, help="Sample size for the generated data.")
    return parser.parse_args()

def main():
    args = parse_args()
    if args.name is None:
        if args.args is not None:
            logger.warning("Ignoring args, generating all test data.")
        for configuration in CONFIG_DICT.values():
            configuration.sample_size = args.size
            generate_test_data(configuration.fct_name.lower(), configuration, args.seed)
    else:
        config = CONFIG_DICT.get(args.name)
        file_name = args.name.lower()
        if args.args is not None:
            config.parameters.update(args.args)
            config.sample_size = args.size
            file_name += f"_{dict_to_posix_filename(args.args)}"
        generate_test_data(file_name, config, args.seed)

if __name__ == "__main__":
    main()
