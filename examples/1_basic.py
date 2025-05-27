from rustfatigue import damage_equiv_load


input = [40.0, 15.0, 72.0, 22.0, 43.0, 82.0, 75.0, 7.0, 34.0, 49.0, 95.0, 75.0, 85.0, 47.0, 63.0, 31.0, 90.0, 20.0, 37.0, 39.0]
wohler, Ncycles = 4, 20

DEL = damage_equiv_load(input, wohler, Ncycles)

print(f"damage equivalent load: {DEL}")