import casadi as cs
import opengen as og
import matplotlib.pyplot as plt
import numpy as np

Th = 30 # Horizon
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
T = 0.2
T2 = 0.2
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
    [states_dot, outputs] = imanishisource_car_long(states, controls)
    return cs.vcat([states[i] + sampling_time * states_dot[i] for i in range(NX)])

def imanishisource_car_long(states, controls):
    # Extract states
    s = states[0]
    sp = states[1]
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
    P_brk = cs.log((omega_e / 100) + 1) / 17
    tau_brk = (P_brk * V_e) / (4 * cs.pi) * 1000
    shafttorque = switching * tau_e - (1 - switching) * tau_brk
    f_roll = mu * m * g
    f_drag = 0.5 * A * C_D * rho * (v**2)
    V_OC = (V_O + k1 * cs.log(xi) + k2 * cs.log(1 - xi) - k3 / xi - k4 * xi) * sel
    v_dot = ((r * G * tau_mg) - (r**2 * (f_roll + f_drag + f_brk))) / (G**2 * j_mg + j_w + m * r**2)
    P_mot = (m * v_dot + f_roll + f_drag + f_brk) * v / 1000
    P_gen = omega_g * tau_g * e_g / 1000

    e_m = 1
    P_c = e_m * P_mot
    P_bat = P_gen - P_c + C

    # State derivatives
    s_dot = v
    sp_dot = vp
    xi_dot = (-V_OC + cs.sqrt(V_OC**2 + 4000 * R_bat * P_bat)) / (2 * R_bat * Q_bat)
    omega_e_dot = (shafttorque - tau_g / id) / (j_e + j_g / (id**2))
    tau_e_dot = (tilde_tau_e * switching - tau_e) / T
    inv_g_dot = (tilde_inv_g - inv_g) / T
    switching_dot = (tilde_switch - switching) / T2
    tau_mg_dot = (tilde_tau_mg - tau_mg) / T

    states_dot = cs.vertcat(s_dot, sp_dot, v_dot, xi_dot, omega_e_dot, tau_e_dot, inv_g_dot, switching_dot, tau_mg_dot)

    # Outputs
    dummyout = 0
    dummy1 = 0
    outputs = cs.vertcat(dummyout, dummy1)

    return states_dot, outputs

def imanishiLagrange_cost(states, controls):
    # Extract outputs
    #dummyout, dummy1 = outputs

    # Extract states
    s = states[0]
    sp = states[1]
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

    # Generator energy conversion efficiency (4th order polynomial)
    pp00 = 84.06
    # ... (other polynomial coefficients)
    pp04 = -7.65e-07

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
    P_e = shafttorque * omega_e / 1000
    f_roll = mu * m * g
    f_drag = 0.5 * A * C_D * rho * (v**2)
    V_OC = (V_O + k1 * np.log(xi) + k2 * np.log(1 - xi) - k3 / xi - k4 * xi) * sel
    v_dot = ((r * G * tau_mg) - (r**2 * (f_roll + f_drag + f_brk))) / (G**2 * j_mg + j_w + m * r**2)
    P_gen = omega_g * tau_g * e_g / 1000
    P_mot = (m * v_dot + f_roll + f_drag + f_brk) * v / 1000

    e_m = 1
    P_c = e_m * P_mot
    P_bat = P_gen - P_c + C

    omega_e_rpm = omega_e * 30 / np.pi
    mdot = (p00 + p10 * omega_e_rpm + p01 * tau_e + p20 * omega_e_rpm**2 + p11 * omega_e_rpm * tau_e +
           p02 * tau_e**2 + p30 * omega_e_rpm**3 + p21 * omega_e_rpm**2 * tau_e +
           p12 * omega_e_rpm * tau_e**2 + p03 * tau_e**3)
    ml = switching * mdot

    xi_dot = (-V_OC + np.sqrt(V_OC**2 + 4000 * R_bat * P_bat)) / (2 * R_bat * Q_bat)
    I = xi_dot * Q_bat  # Current [A]

    # Weights
    w1 = 10
    w2 = 1
    w3 = 1e-3
    w4 = 1e-4
    w5 = 1

    # Cost function components
    P_f = ml * He
    W_e = P_f - P_e
    W_bat = I**2 * R_bat / 1000  # Battery consumption energy [kW]
    W_gen = (1 - e_g) * (P_gen / e_g)
    c_s = switching - switching**2
    sig_w = 0.5 * (np.tanh(omega_e - 95) + 1)
    cost_w = switching * (1 - sig_w)
    D = ((sp - s) - h * vp)**2

    # Cost function calculation
    car_Lagrange = w5 * (W_e + W_bat + W_gen) + w1 * c_s + w2 * cost_w + w3 * D + w4 * (tilde_tau_e**2 + tilde_tau_mg**2)
    #costs = [car_Lagrange]

    return car_Lagrange

states_min = [0,0,0.01/3.6,0.2*0.01,0,0,0,0,-254*0.01]
states_max = [cs.inf,cs.inf,1/3.6,0.8*0.01,565*0.01,103*0.01,1,1,254*0.01]

states_min_vector = cs.vcat(states_min)
states_max_vector = cs.vcat(states_max)

def state_constraint(states, controls):
    states_t = states
    for k in range(Th):
        states_t = dynamic_dt(states_t, controls[k])
        constraint_list = []
        for i in range(states_t.size1()):
            constraint_list.append(cs.fmax(states_min[i], states_t[i]) - states_t[i])
            constraint_list.append(states_t[i] - cs.fmin(states_max[i], states_t[i]))
    return cs.vcat(constraint_list)

states_0 = cs.MX.sym("states_0", NX)
controls_k = [cs.MX.sym('controls_' + str(i), NU) for i in range(Th)]

states_t = states_0
total_cost = 0
for k in range(0, Th):
    total_cost += imanishiLagrange_cost(states_t, controls_k[k])
    states_t = dynamic_dt(states_t, controls_k[k])

control_min = [0,0,0,-254*0.01]
control_max = [103*0.01, 1,1,254*0.01]

f_pm = state_constraint(states_0, controls_k)

input_bound = og.constraints.Rectangle(control_min*Th, control_max*Th)

optimization_variables = []
optimization_parameters = []

optimization_variables += controls_k
optimization_parameters += [states_0]
optimization_variables = cs.vertcat(*optimization_variables)
optimization_parameters = cs.vertcat(*optimization_parameters)

problem = og.builder.Problem(optimization_variables,
                             optimization_parameters,
                             total_cost) \
    .with_penalty_constraints(f_pm) \
    .with_constraints(input_bound)

build_config = og.config.BuildConfiguration()  \
    .with_build_directory("python_build")      \
    .with_build_mode("release")                \
    .with_tcp_interface_config()

meta = og.config.OptimizerMeta().with_optimizer_name("guan_optimizer")

solver_config = og.config.SolverConfiguration()\
    .with_initial_tolerance(10)

builder = og.builder.OpEnOptimizerBuilder(problem, meta,
                                          build_config, solver_config)
builder.build()