def mass_to_fuel_req(mass : int):
    return int(mass / 3) - 2

def get_total_fuel_for_mass(mass):
    fuel = 0
    while mass > 0:
        fuel_req = mass_to_fuel_req(mass)
        if fuel_req <= 0:
            break

        fuel += fuel_req
        mass = fuel_req

    return fuel

text = []
with open("input.txt", "r") as file:
    text = file.read().splitlines()

counter = 0
for line in text:
    fuel = get_total_fuel_for_mass(int(line))
    counter += fuel


print(counter)