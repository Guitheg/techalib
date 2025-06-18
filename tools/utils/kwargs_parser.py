import argparse
from utils.logger import logger

class ParseKwargs(argparse.Action):
    def __call__(self, parser, namespace, values, option_string=None):
        setattr(namespace, self.dest, dict())
        for value in values:
            key, value = value.split('=')
            if "." in value:
                try:
                    value = float(value)
                except ValueError:
                    logger.warning(f"Could not convert {value} to float, keeping as string.")
            elif value.isdigit():
                try:
                    value = int(value)
                except ValueError:
                    logger.warning(f"Could not convert {value} to int, keeping as string.")
            elif value.lower() in ['true', 'false']:
                value = value.lower() == 'true'
            getattr(namespace, self.dest)[key] = value
