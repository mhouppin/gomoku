class InstanceMenuExecption(Exception):
    """Exception raised when the instance of the menu is not created"""

    def __init__(self, message: str) -> None:
        super().__init__(message)


class PlayerAlreadySetException(Exception):
    """Exception raised when the players are already set"""

    def __init__(self) -> None:
        super().__init__("Players already set")
