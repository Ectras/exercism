from collections import Counter
import re


def count_words(sentence: str) -> Counter[str]:
    return Counter(re.findall(r"\d+|\w+(?:'\w+)?", sentence.lower().replace("_", " ")))
