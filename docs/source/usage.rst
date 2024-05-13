Usage
=====

.. _installation:

Installation
------------

1. First grab the latest version from the GitHub `releases page <https://github.com>`_.
2. Download two copies of the `NASR data <https://www.faa.gov/air_traffic/flight_info/aeronav/Aero_Data/NASR_Subscription>`_ from different data cycles. APT and PFR copies are required for the application to fully function.
3. Create a folder in the same as the executable called "data". Alternatively, run the program and it will create it for you
4. Extract the APT_BASE.csv and PFR_BASE.csv files from the **newer** nav data cycle into the data folder.
5. Rename these files to APT_BASE_NEW.csv and PFR_BASE_NEW.csv
6. Extract the APT_BASE.csv and PFR_BASE.csv from the **older** nav data cycle into the data folder.
7. Run the program and enjoy.

Program functions
-----------------
When you start the program, you will be presented with the main menu.

 | 1) Export airport list
 | 2) Generate airport changes list
 | 3) Generate TEC Route changes list
 | 4) Export TEC Routes
 | 0) Quit

Enter the relevant number to access the desired function.

Export airport list
^^^^^^^^^^^^^^^^^^^
This function is used to output the required CSV format for MyFlightRoute.

No user input is required.

Generate airport changes list
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
This will output a list of airports that are due to open, close or be renamed in the next data cycle, and output them formatted for Discord Markdown.

No user input is required.

Generate TEC Route changes list
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
This will output a list of Californian TEC Routes due to be created, deleted or modified in the next air data cycle. Changes tracked are Altitude and Route String.

No user input is required.

Export TEC Routes
^^^^^^^^^^^^^^^^^
This function outputs the required TEC Route data in the required CSV format for MyFlightRoute.

No user input is required.