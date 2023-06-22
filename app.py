from data_processing import DataProcesser
import environment
import os
from colorama import Fore, Style
import time


class App:

    def __init__(self) -> None:
        # self.__processer = DataProcesser()
        pass


    def get_user_input(self):
        while True:
            print("""
Enter the action you want to take:
0) exit
1) Show average humidity
2) Show average temperature per hour
3) Show average light-level 
4) Show average per-day
5) Show live data
6) Show all devices
                """)
            
            decision: int
            try:
                decision = int(input())
            except Exception:
                continue

            match decision:
                case 0:
                    print("Exiting...")
                    os._exit(0)
                case 1:
                    self.show_average_humidity()
                case 2:
                    self.show_average_temperature()
                case 3:
                    self.show_average_light_level()
                case 4:
                    self.show_average_data_per_day()
                case 5:
                    self.show_live_data()
                case 6:
                    self.show_all_devices()


    def show_average_humidity(self):
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start: str = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end: str = input()
        data: float = self.__processer.get_average_humidity(start, end)

        print(f"{data: .2f}%")


    def show_average_temperature(self) -> None:
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start: str = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end: str = input()

        if not(start and end):
            print("Invalid input!")
            return
        data: float = self.__processer.get_average_temperature(start, end)

        print(f"{data: .2f}°C")
        
    def show_average_light_level(self) -> None:
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start: str = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end: str = input()

        if not(start and end):
            print("Invalid input!")
            return
        data = self.__processer.get_average_light_level(start, end)
        print(f"{data} lux")


    def show_average_data_per_day(self):
        print("""Enter the start-date of the period (format: '2022-01-01 00:00:00'):""")
        start: str = input()

        print("""Enter the end-date of the period (format: '2022-01-01 00:00:00'):""")
        end: str = input()

        if not(start and end):
            print("Invalid input!")
            return

        data = self.__processer.get_average_data_per_day(start, end)

        for element in data:
            print(f"{Fore.GREEN}{element.sensor_id}:{Style.RESET_ALL}")
            print(f"    Date:           {element.date}")
            if element.average_temperature:
                print(f"    Temperature:    {element.average_temperature: .2f}°C")
            if element.average_humidity:
                print(f"    Humidity:       {element.average_humidity: 2f}%")
            if element.average_light_level:
                print(f"    Light-Level:    {element.average_light_level} lux")

    def show_live_data(self) -> None:
        def __loop_live_data_output(name):
            while True:
                time.sleep(0.1)
                data: dict[str, any] = self.__processer.get_most_recent_data(name)

                if not data:
                    print(f"{Fore.RED}Device does not exist!{Style.RESET_ALL}\nValid devices are:")
                    self.show_all_devices()
                    break
                os.system('cls' if os.name == 'nt' else 'clear')

                print(f"{Fore.GREEN}Most Recent Data: {Style.RESET_ALL}")
                if data.temperature:
                    print(f"    Temperature:    {data.temperature: .2f}°C")
                if data.humidity:
                    print(f"    Humidity:       {data.humidity: .2f}%")
                if data.light_level:
                    print(f"    Light-Level:    {data.light_level} lux")

        print("Enter the device-name: ")
        name: str = input()
        
        if not name:
            print("Invalid Input!")
            return

        try:
            __loop_live_data_output(name)
        except KeyboardInterrupt:
            print("Stopping live data...")

    def show_all_devices(self) -> None:
        data: dict[str, int] = self.__processer.get_all_devices()

        print(f"{Fore.GREEN}Devices:{Style.RESET_ALL}")
        for element in data:
            print("\t" + element.sensor_id)



if __name__ == "__main__":
    environment.run()

    app = App()
    app.get_user_input()