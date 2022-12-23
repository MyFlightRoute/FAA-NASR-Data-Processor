from python_console_menu import AbstractMenu, MenuItem


class MainMenu(AbstractMenu):
    show_hidden_menu = False

    def __init__(self):
        super().__init__("Welcome to CIFP Data Processor")

    def initialise(self):
        self.add_menu_item(MenuItem(1, "Download Dataset"))
        self.add_menu_item(MenuItem(2, "Exit").set_as_exit_option())


main_menu = MainMenu()
main_menu.display()
