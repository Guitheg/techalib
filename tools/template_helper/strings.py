T = "    "  # Tab character for indentation
T2 = T+T
def format_tab(str: str, *args, **kwargs) -> str:
    return str.format(
        T=T,
        T2=T2,
        *args, **kwargs
    )
