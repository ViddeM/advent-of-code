from dataclasses import dataclass


@dataclass
class Passport:
    byr: str
    iyr: str
    eyr: str
    hgt: str
    hcl: str
    ecl: str
    pid: str

    def validate_values(self) -> bool:
        byr = validate_number(self.byr)
        if byr < 1920 or byr > 2002:
            print(f"INVALID BIRTH YEAR {self.byr}")
            return False

        iyr = validate_number(self.iyr)
        if iyr < 2010 or iyr > 2020:
            print(f"INVALID ISSUE YEAR {self.iyr}")
            return False

        eyr = validate_number(self.eyr)
        if eyr < 2020 or eyr > 2030:
            print(f"INVALID EXPIRATION YEAR {self.eyr}")
            return False

        hgt, hgt_unit = validate_number_with_unit(self.hgt)
        if hgt_unit == "cm":
            if hgt < 150 or hgt > 193:
                print(f"INVALID HEIGHT {self.hgt}")
                return False
        elif hgt_unit == "in":
            if hgt < 59 or hgt > 76:
                print(f"INVALID HEIGHT {self.hgt}")
                return False
        else:
            print(f"INVALID HEIGHT {self.hgt}")
            return False

        if not validate_hex(self.hcl):
            print(f"INVALID HAIR COLOR {self.hcl}")
            return False

        valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        if self.ecl not in valid_eye_colors:
            print(f"INVALID EYE COLOR {self.ecl}")
            return False

        if len(self.pid) != 9 or validate_number(self.pid) <= 0:
            print(f"INVALID PASSPORT ID {self.pid}")
            return False

        return True


def validate_number(number: str) -> int:
    try:
        num = int(number)
        return num
    except ValueError:
        return -1


def validate_hex_number(number: str) -> bool:
    try:
        num = int(number, 16)
        return num
    except ValueError:
        return -1


def validate_number_with_unit(text: str) -> (int, str):
    unit = ""
    number = ""
    for char in reversed(text):
        if char.isdigit():
            number = char + number
        else:
            unit = char + unit

    return validate_number(number), unit


def validate_hex(text: str) -> bool:
    if len(text) != 7:
        return False

    if text[0] != "#":
        return False

    return validate_hex_number(text[1:])
