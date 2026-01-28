class Queen:
    def __init__(self, row: int, column: int):
        if row < 0:
            raise ValueError("row not positive")
        if row >= 8:
            raise ValueError("row not on board")
        if column < 0:
            raise ValueError("column not positive")
        if column >= 8:
            raise ValueError("column not on board")
        self.row = row
        self.column = column

    def can_attack(self, another_queen: "Queen") -> bool:
        if (self.row, self.column) == (another_queen.row, another_queen.column):
            raise ValueError("Invalid queen position: both queens in the same square")
        return (
            self.row == another_queen.row
            or self.column == another_queen.column
            or abs(self.row - another_queen.row)
            == abs(self.column - another_queen.column)
        )
