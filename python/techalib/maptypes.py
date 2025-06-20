from collections import namedtuple

FCT_TO_NAMEDTUPLE = {
    "aroonosc": namedtuple("AroonoscResult", ["aroonosc", "state"]),
    "aroon": namedtuple("AroonResult", ["aroon_down", "aroon_up", "state"]),
    "rocr": namedtuple("RocrResult", ["rocr", "state"]),
    "adx": namedtuple("AdxResult", ["adx", "state"]),
    "dx": namedtuple("DxResult", ["dx", "state"]),
    "plus_di": namedtuple("Plus_diResult", ["plus_di", "state"]),
    "minus_di": namedtuple("Minus_diResult", ["minus_di", "state"]),
    "plus_dm": namedtuple("Plus_dmResult", ["plus_dm", "state"]),
    "minus_dm": namedtuple("Minus_dmResult", ["minus_dm", "state"]),
    "ad": namedtuple("AdResult", ["ad", "state"]),
    "atr": namedtuple("AtrResult", ["atr", "state"]),
    "roc": namedtuple("RocResult", ["roc", "state"]),
    "midprice": namedtuple("MidpriceResult", ["midprice", "state"]),
    "midpoint": namedtuple("MidpointResult", ["midpoint" ,"state"]),
    "kama": namedtuple("KamaResult", ["kama" ,"state"]),
    "t3": namedtuple("T3Result", ["t3" ,"state"]),
    "trima": namedtuple("TrimaResult", ["trima", "state"]),
    "tema": namedtuple("TemaResult", ["tema" ,"state"]),
    "dema": namedtuple("DemaResult", ["dema" ,"state"]),
    "wma": namedtuple("WmaResult", ["wma", "state"]),
    "bbands": namedtuple("BbandsResult", ["upper", "middle", "lower", "state"]),
    "ema": namedtuple("EmaResult", ["ema", "state"]),
    "sma": namedtuple("SmaResult", ["sma", "state"]),
    "rsi": namedtuple("RsiResult", ["rsi", "state"]),
    "macd": namedtuple("MacdResult", ["macd", "signal", "histogram", "state"]),
}

def __tuple2types__(function, result: tuple) -> object:
    tech_result = FCT_TO_NAMEDTUPLE.get(function.__name__)
    if tech_result is None:
        return result
    return tech_result(*result)
