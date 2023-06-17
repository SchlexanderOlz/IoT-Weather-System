from data_processing import DataProcesser
import environment
from server import Server
import os


# TODO Find a better sollution for serving a front-end API
class App:

    def __init__(self) -> None:
        self.__processer = DataProcesser([('localhost', 9042)])


    def get_user_input(self):
        while True:
            print("""
Enter the action you want to take:
0) exit
1) Show average humidity
2) Show average temperature per hour
3) Display everything available
                """)

            match int(input()):
                case 0:
                    print("Exiting...")
                    os._exit(0)
                case 1:
                    self.display_average_humidity()
                case 2:
                    self.get_average_temperature()


    def display_average_humidity(self):
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end = input()

        try:
            data = self.__processer.get_average_humidity(start, end)
        except Exception as e:
            print(e)
            print("Invalid Input!")
            return

        print(data)


    def get_average_temperature(self):
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end = input()

        try:
            data = self.__processer.get_average_temperature(start, end)
        except Exception as e:
            print(e)
            print("Invalid Input!")
            return

        print(data)

if __name__ == "__main__":
    environment.run()

    app = App()
    app.get_user_input()