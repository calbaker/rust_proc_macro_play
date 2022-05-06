import core

train_sim = core.TrainSimulation()
train_sim.walk()

print(train_sim.loco_con.fc.pwr_max_watts == 100)

try:
    train_sim.loco_con.fc.pwr_max_watts = 77
except:
    print("This was an expected failure!")
