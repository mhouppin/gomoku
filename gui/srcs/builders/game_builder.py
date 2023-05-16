import random
from srcs.models.game import Game
from srcs.models import HumanPlayer, AiPlayer


class GameBuilder:
    """This class is responsible for building the game."""

    game: Game

    def __init__(self) -> None:
        self.game = Game()

    def reset(self) -> "GameBuilder":
        """Reset the game."""
        self.game = Game()
        return self

    def set_turn_time(self, turn_time: int) -> "GameBuilder":
        """Set the turn time."""
        self.game.set_turn_time(turn_time)
        return self

    def set_game_time(self, game_time: int) -> "GameBuilder":
        """Set the game time."""
        self.game.set_game_time(game_time)
        return self

    def set_players_only_humans(self) -> "GameBuilder":
        """Set the players to only humans."""

        self.game.set_players(HumanPlayer(), HumanPlayer())
        return self

    def set_players_only_ai(self) -> "GameBuilder":
        """Set the players to only AI."""

        self.game.set_players(AiPlayer(), AiPlayer())
        return self

    def set_players_both_ai_human(self) -> "GameBuilder":
        """Set the players to AI and human. Randomly choose who starts the game."""

        if random.randint(0, 1):
            self.game.set_players(AiPlayer(), HumanPlayer())
        else:
            self.game.set_players(HumanPlayer(), AiPlayer())

        return self

    def build(self) -> Game:
        """Build the game."""
        return self.game
