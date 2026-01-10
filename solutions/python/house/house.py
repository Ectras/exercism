NOUNS_AND_VERBS = [
    ("malt", "lay in"),
    ("rat", "ate"),
    ("cat", "killed"),
    ("dog", "worried"),
    ("cow with the crumpled horn", "tossed"),
    ("maiden all forlorn", "milked"),
    ("man all tattered and torn", "kissed"),
    ("priest all shaven and shorn", "married"),
    ("rooster that crowed in the morn", "woke"),
    ("farmer sowing his corn", "kept"),
    ("horse and the hound and the horn", "belonged to"),
]


def get_verse(verse: int) -> str:
    # This is the <thing1> that <verb1> the <thing2> that <verb2> the <thing3>
    out = "This is "
    for thing, verb in reversed(NOUNS_AND_VERBS[: verse - 1]):
        out += f"the {thing} that {verb} "
    out += "the house that Jack built."
    return out


def recite(start_verse: int, end_verse: int):
    return [get_verse(i) for i in range(start_verse, end_verse + 1)]
