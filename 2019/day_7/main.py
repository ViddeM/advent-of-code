from day_7 import intcode

outputs = {}
filename = "input.txt"
for a in range(0, 5):
    print("processing A = " + str(a))
    a_out = intcode.run_program(filename, [a, 0])
    for b in range(0, 5):
        if b == a:
            continue
        print("processing B = " + str(b))
        b_out = intcode.run_program(filename, [b, a_out])

        for c in range(0, 5):
            if c == a or c == b:
                continue
            print("processing C = " + str(c))
            c_out = intcode.run_program(filename, [c, b_out])

            for d in range(0, 5):
                if d == a or d == b or d == c:
                    continue

                print("processing D = " + str(d))
                d_out = intcode.run_program(filename, [d, c_out])

                for e in range(0, 5):
                    if e == a or e == b or e == c or e == d:
                        continue

                    print("processing E = " + str(e))
                    e_out = intcode.run_program(filename, [e, d_out])
                    outputs[(a, b, c, d, e)] = e_out


print(outputs)