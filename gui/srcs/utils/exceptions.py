class InstanceMenuExecption(Exception):
    """Exception raised when the instance of the menu is not created"""

    def __init__(self, message: str) -> None:
        super().__init__(message)
