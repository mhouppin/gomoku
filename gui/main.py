"""GUI commands"""
from pprint import pprint
import click

from srcs.board import Board
from srcs.game import Game


@click.group()
def gui() -> None:
    """GUI commands"""


@gui.command()
def run() -> None:
    """Run the GUI"""
    game = Game()
    board = Board()
    pprint(board.grid.rows)

    game.run()


if __name__ == "__main__":
    gui()
