import opengen as og
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np

mng = og.tcp.OptimizerTcpManager("python_build/guan_optimizer")
mng.start()

mps2kmph = 3.6
x_state_0 = [0,20, 20/mps2kmph, 0.3, 0, 0, 0, 0, 0]

simulation_steps = 3000

state_sequence = x_state_0
input_sequence = []

NU = 4
NX = 9
sampling_time = 0.1

# Parameters
id = 0.6
C = 0
mu = 0.01
m = 1465
g = 9.8
rho = 1.23
C_D = 0.3
A = 1.746
Q_bat = 3.6 * 10**3 * 4.47
R_bat = 1.2
T = 0.1
T2 = 0.1
r = 0.287
G = 7.9377
f_brk = 0
j_e = 0.18
j_g = 0.07
j_mg = 3.3807
j_w = 0.0226
vp = 60 / 3.6
V_e = 1.198
V_O = 4.14
k1 = 0.237
k2 = -0.0516
k3 = 0.00105
k4 = 0.183
sel = 80            #セルの個数N
h = 2.2              # 車間時間[s]
He = 34.6           #ガソリンの燃焼熱[MJ/kg]

# Generator energy conversion efficiency (4th order polynomial)
pp00 =       84.06
pp10 =   0.0005904
pp01 =      0.3328
pp20 =   9.904e-08
pp11 =   0.0001827
pp02 =    -0.01527
pp30 =   -4.11e-11
pp21 =  -2.351e-08
pp12 =  -1.327e-06
pp03 =   0.0001891
pp40 =   2.072e-15
pp31 =    1.04e-12
pp22 =   7.855e-11
pp13 =    3.09e-09
pp04 =   -7.65e-07

p00 =           0
p10 =   0.0001265
p01 =     0.02197
p20 =  -1.527e-08
p11 =   1.916e-06
p02 =  -0.0005993
p30 =   2.076e-12
p21 =   5.074e-10
p12 =   7.772e-09
p03 =   4.646e-06

def dynamic_dt(states, controls):
    states_dot = imanishisource_car_long(states, controls)
    return [states[i] + sampling_time * states_dot[i] for i in range(NX)]

def imanishisource_car_long(states, controls):
    # Extract states
    v = states[2]
    xi = states[3]
    omega_e = states[4]
    tau_e = states[5]
    inv_g = states[6]
    switching = states[7]
    tau_mg = states[8]

    # Extract controls
    tilde_tau_e = controls[0]
    tilde_inv_g = controls[1]
    tilde_switch = controls[2]
    tilde_tau_mg = controls[3]

    # Generator torque calculations
    omega_g = omega_e / id
    omega_g_rpm = omega_g * 30 / np.pi
    tau_g = 2 * omega_g * (omega_g / 1000 + 1) * np.exp(-(omega_g / 300 + 1)) * inv_g

    # Energy conversion map (4th order polynomial)
    e_g_org = (pp00 + pp10 * omega_g_rpm + pp01 * tau_g + 
               pp20 * omega_g_rpm**2 + pp11 * omega_g_rpm * tau_g + 
               pp02 * tau_g**2 + pp30 * omega_g_rpm**3 + 
               pp21 * omega_g_rpm**2 * tau_g + pp12 * omega_g_rpm * tau_g**2 + 
               pp03 * tau_g**3 + pp40 * omega_g_rpm**4 + 
               pp31 * omega_g_rpm**3 * tau_g + pp22 * omega_g_rpm**2 * tau_g**2 + 
               pp13 * omega_g_rpm * tau_g**3 + pp04 * tau_g**4)
    e_g = e_g_org * 0.01

    # Other calculations
    P_brk = np.log((omega_e / 100) + 1) / 17
    tau_brk = (P_brk * V_e) / (4 * np.pi) * 1000
    shafttorque = switching * tau_e - (1 - switching) * tau_brk
    f_roll = mu * m * g
    f_drag = 0.5 * A * C_D * rho * (v**2)
    V_OC = (V_O + k1 * np.log(xi) + k2 * np.log(1 - xi) - k3 / xi - k4 * xi) * sel
    v_dot = ((r * G * tau_mg) - (r**2 * (f_roll + f_drag + f_brk))) / (G**2 * j_mg + j_w + m * r**2)
    P_mot = (m * v_dot + f_roll + f_drag + f_brk) * v / 1000
    P_gen = omega_g * tau_g * e_g / 1000

    e_m = 1
    P_c = e_m * P_mot
    P_bat = P_gen - P_c + C

    # State derivatives
    s_dot = v
    sp_dot = vp
    xi_dot = (-V_OC + np.sqrt((V_OC)**2 + 4000 * R_bat * P_bat)) / (2 * R_bat * Q_bat)
    # Continuation of state derivatives
    omega_e_dot = (shafttorque - tau_g / id) / (j_e + j_g / (id**2))
    tau_e_dot = (tilde_tau_e * switching - tau_e) / T
    inv_g_dot = (tilde_inv_g - inv_g) / T
    switching_dot = (tilde_switch - switching) / T2
    tau_mg_dot = (tilde_tau_mg - tau_mg) / T

    states_dot = [s_dot, sp_dot, v_dot, xi_dot, omega_e_dot, tau_e_dot, inv_g_dot, switching_dot, tau_mg_dot]
    return states_dot

x = x_state_0
for k in range(simulation_steps):
    solver_status = mng.call(x)
    us = solver_status['solution']
    u = us[0:NU]
    x_next = dynamic_dt(x, u)
    state_sequence += x_next
    input_sequence += u
    x = x_next

mng.kill()

time = np.arange(0, sampling_time*simulation_steps, sampling_time)

s  = state_sequence[0:NX*simulation_steps:NX]
sp = state_sequence[1:NX*simulation_steps+1:NX]
v  = state_sequence[2:NX*simulation_steps+2:NX]
xi = state_sequence[3:NX*simulation_steps+3:NX]
omega_e= state_sequence[4:NX*simulation_steps+4:NX]
tau_e = state_sequence[5:NX*simulation_steps+5:NX]
inv_g = state_sequence[6:NX*simulation_steps+6:NX]
switching = state_sequence[7:NX*simulation_steps+7:NX]
tau_mg = state_sequence[8:NX*simulation_steps+8:NX]

# s_p - s
B = [a - b for a, b in zip(sp, s)]

u_1 = input_sequence[0:NU*simulation_steps:NU]
u_2 = input_sequence[1:NU*simulation_steps+1:NU]
u_3 = input_sequence[2:NU*simulation_steps+2:NU]
u_4 = input_sequence[3:NU*simulation_steps+3:NU]


plt.figure(figsize=(10, 8))

# Plot each state in a subplot
plt.subplot(3, 3, 1)
plt.plot(time, B)
plt.title('sp-s')

plt.subplot(3, 3, 2)
plt.plot(time, v)
plt.title('v')

plt.subplot(3, 3, 3)
plt.plot(time, xi)
plt.title('xi')

plt.subplot(3, 3, 4)
plt.plot(time, inv_g)
plt.title('inv_g')

plt.subplot(3, 3, 5)
plt.plot(time, switching)
plt.title('switching')

plt.subplot(3, 3, 6)
plt.plot(time, tau_mg)
plt.title('tau_mg')

plt.subplot(3, 3, 7)
plt.plot(time, u_2)
plt.title('tilde_inv_g')

plt.subplot(3, 3, 8)
plt.plot(time, u_3)
plt.title('tilde_switch')

plt.subplot(3, 3, 9)
plt.plot(time, u_4)
plt.title('tilde_tau_mg')

plt.tight_layout()
plt.show()