import core

train_sim = core.TrainSimulation()
train_sim.walk()

print(train_sim.loco_con.fc.pwr_max_watts == 100)