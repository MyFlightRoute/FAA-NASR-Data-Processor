from python_console_menu import AbstractMenu, MenuItem
from tqdm import tqdm
import os
import time


def initialise_data():

    does_cifp_faacifp18_file_exist = os.path.exists('data/FAACIFP18')
    does_cifp_in_cifp_file_exist = os.path.exists('data/IN_CIFP.txt')

    if not does_cifp_faacifp18_file_exist:
        print("CIFP FAACIFP18 file missing")
    else:
        print("CIFP FAACIFP18 file exists")

    if not does_cifp_in_cifp_file_exist:
        print("CIFP IN_CIFP file missing")

    time.sleep(2)


class MainMenu(AbstractMenu):
    show_hidden_menu = False

    def __init__(self):
        super().__init__("Welcome to CIFP Data Processor")

    def initialise(self):
        self.add_menu_item(MenuItem(1, "Download Dataset"))
        self.add_menu_item(MenuItem(2, "Check prerequisites for data processing", lambda: initialise_data()))
        self.add_menu_item(MenuItem(3, "Exit").set_as_exit_option())


def extract_airport_list():
    dataset = open('data/FAACIFP18', 'r')
    raw_data = dataset.readlines()

    # print(raw_data)

    raw_airports = []

    for i in tqdm(range(0, len(raw_data)), desc="Progress"):
        if "SUSAP" in raw_data[i]:
            raw_airports += raw_data

    print(raw_airports)


extract_airport_list()

# main_menu = MainMenu()
# main_menu.display()
