"""Module unit tests for Game"""
from srcs.models.app import App


class TestGame:
    """Test class for Game"""

    def test_construct_game_works(self) -> None:
        """Test if the constructor of Game works"""
        game = App()
        assert game.is_running is False
