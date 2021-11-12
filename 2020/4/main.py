from passport import Passport

passports = []
required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
optional_keys = ["cid"]

with open("input.txt", "r") as file:
    lines = file.readlines()
    curr_passport = {}
    for l in lines:
        line = l.replace("\n", "")
        if line == "":
            passports.append(curr_passport)
            curr_passport = {}
        else:
            entries = line.split(" ")
            for entry in entries:
                key_val = entry.split(":")
                num_key_vals = len(key_val)
                if num_key_vals != 2:
                    print(f"ERR Invalid key_val {key_val} with length {num_key_vals}!")
                    exit(1)

                curr_passport[key_val[0]] = key_val[1]

    if curr_passport != {}:
        passports.append(curr_passport)

# pprint(passports)


def validate_passport(passport, required_keys):
    for required_key in required_keys:
        if required_key not in passport:
            return False

    return True


def validate_passport_data(passport) -> Passport:
    return Passport(
        byr=passport["byr"],
        iyr=passport["iyr"],
        eyr=passport["eyr"],
        hgt=passport["hgt"],
        hcl=passport["hcl"],
        ecl=passport["ecl"],
        pid=passport["pid"]
    )


def validate_passports(passports, required_keys):
    valid = 0
    valid_data = 0
    for passport in passports:
        if validate_passport(passport, required_keys):
            valid += 1

            validated_passport = validate_passport_data(passport)
            if validated_passport.validate_values():
                valid_data += 1

    print(f"Part One -- There are {valid} valid passports")
    print(f"Part Two -- There are {valid_data} valid passports")


validate_passports(passports, required_keys)
