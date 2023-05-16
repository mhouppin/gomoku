from srcs.utils.exceptions import PlayerAlreadySetException
from srcs.interfaces import Player
from srcs.models import Move


class Game:
    """This class represents a game."""

    __players: tuple[Player, Player] | None
    __current_player: Player
    __histories: list[Move]
    __turn_count: int
    __turn_time: int
    __game_time: int

    def __init__(self) -> None:
        self.__states = []
        self.__players = None
        self.__turn_count = 0
        self.__turn_time = 1000 * 60
        self.__game_time = 1000 * 60 * 10

    def set_turn_time(self, turn_time: int) -> None:
        self.__turn_time = turn_time

    def set_game_time(self, game_time: int) -> None:
        self.__game_time = game_time

    def set_players(self, player1: Player, player2: Player) -> None:
        if self.__players:
            raise PlayerAlreadySetException()
        self.__players = (player1, player2)
        self.__current_player = player1
