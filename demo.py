import core

train_sim = core.TrainSimulation()
train_sim.walk()

print("value as expected:", train_sim.loco_con.fc.pwr_max_watts == 100)

try:
    train_sim.loco_con.fc.pwr_max_watts = 77
except Exception as e:
    print(e)
    print("This was an expected failure!")
