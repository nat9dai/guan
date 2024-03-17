import casadi as cs
import opengen as og

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
pp20 =   9.904e-8
pp11 =   0.0001827
pp02 =    -0.01527
pp30 =   -4.11e-11
pp21 =  -2.351e-8
pp12 =  -1.327e-6
pp03 =   0.0001891
pp40 =   2.072e-15
pp31 =    1.04e-12
pp22 =   7.855e-11
pp13 =    3.09e-9
pp04 =   -7.65e-7

p00 =           0
p10 =   0.0001265
p01 =     0.02197
p20 =  -1.527e-8
p11 =   1.916e-6
p02 =  -0.0005993
p30 =   2.076e-12
p21 =   5.074e-10
p12 =   7.772e-9
p03 =   4.646e-6

# Weights
w1 = 10
w2 = 1
w3 = 1e-3
w4 = 1e-4
w5 = 1

B_min = -30
B_max = 30

P_batmin=-80    #バッテリーの最小流入量[kW]
P_batmax=80     #バッテリーの最大流入量[kW]

def dynamic_dt(states, controls):
    states_dot = imanishisource_car_long(states, controls)
    return cs.vcat([states[i] + sampling_time * states_dot[i] for i in range(NX)])

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
    omega_g_rpm = omega_g * 30 / cs.pi
    tau_g = 2 * omega_g * (omega_g / 1000 + 1) * cs.exp(-(omega_g / 300 + 1)) * inv_g

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

    states_dot = [s_dot, sp_dot, v_dot, xi_dot, omega_e_dot, tau_e_dot, inv_g_dot, switching_dot, tau_mg_dot]
    states_dot = cs.vertcat(*states_dot)

    return states_dot

def imanishiLagrange_cost(states, controls):
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
    tilde_tau_mg = controls[3]

    # Generator torque calculations
    omega_g = omega_e / id
    omega_g_rpm = omega_g * 30 / cs.pi
    tau_g = 2 * omega_g * (omega_g / 1000 + 1) * cs.exp(-(omega_g / 300 + 1)) * inv_g

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
    P_e = shafttorque * omega_e / 1000
    f_roll = mu * m * g
    f_drag = 0.5 * A * C_D * rho * (v**2)
    V_OC = (V_O + k1 * cs.log(xi) + k2 * cs.log(1 - xi) - k3 / xi - k4 * xi) * sel
    v_dot = ((r * G * tau_mg) - (r**2 * (f_roll + f_drag + f_brk))) / (G**2 * j_mg + j_w + m * r**2)
    P_gen = omega_g * tau_g * e_g / 1000
    P_mot = (m * v_dot + f_roll + f_drag + f_brk) * v / 1000

    e_m = 1
    P_c = e_m * P_mot
    P_bat = P_gen - P_c + C

    omega_e_rpm = omega_e * 30 / cs.pi
    mdot = (p00 + p10 * omega_e_rpm + p01 * tau_e + p20 * omega_e_rpm**2 + p11 * omega_e_rpm * tau_e +
           p02 * tau_e**2 + p30 * omega_e_rpm**3 + p21 * omega_e_rpm**2 * tau_e +
           p12 * omega_e_rpm * tau_e**2 + p03 * tau_e**3)
    ml = switching * mdot

    xi_dot = (-V_OC + cs.sqrt(V_OC**2 + 4000 * R_bat * P_bat)) / (2 * R_bat * Q_bat)
    I = xi_dot * Q_bat  # Current [A]

    # Cost function components
    P_f = ml * He
    W_e = P_f - P_e
    W_bat = I**2 * R_bat / 1000  # Battery consumption energy [kW]
    W_gen = (1 - e_g) * (P_gen / e_g)
    c_s = switching - switching**2
    sig_w = 0.5 * (cs.tanh(omega_e - 95) + 1)
    cost_w = switching * (1 - sig_w)
    D = ((sp - s) - h * vp)**2

    # Cost function calculation
    car_Lagrange = w5 * (W_e + W_bat + W_gen) + w1 * c_s + w2 * cost_w + w3 * D + w4 * (tilde_tau_e**2 + tilde_tau_mg**2)

    return car_Lagrange

def penaltyConstraints(states, controls):
    f2 = []
    x = states
    for k in range(Th):
        x = dynamic_dt(x, controls[k])
        v = x[2]
        inv_g = x[6]
        switching = x[7]
        s = x[0]
        sp = x[1]
        f2_list = [inv_g, switching, inv_g-1, switching-1, cs.fmax(0, 30 - sp + s)]
        f2 = f2 + f2_list

    return cs.vertcat(*f2)

def state_constraint(states, controls):
    states_min = [0,0,1/3.6,0.2,0,0,0,0,-254, P_batmin]
    states_max = [9999,9999,100/3.6,0.8,565,103,1,1,254, P_batmax]
    states_bound = og.constraints.Rectangle(states_min*Th, states_max*Th)
    states_t = states
    f1 = []
    for k in range(Th):
        states_t = dynamic_dt(states_t, controls[k])
        f1.append(states_t)
        v = states_t[2]
        omega_e = states_t[4]
        inv_g = states_t[6]
        tau_mg = states_t[8]

        omega_g = omega_e / id
        tau_g = 2 * omega_g * (omega_g / 1000 + 1) * cs.exp(-(omega_g / 300 + 1)) * inv_g
        omega_g_rpm = omega_g * 30 / cs.pi
        # Energy conversion map (4th order polynomial)
        e_g_org = (pp00 + pp10 * omega_g_rpm + pp01 * tau_g + 
                pp20 * omega_g_rpm**2 + pp11 * omega_g_rpm * tau_g + 
                pp02 * tau_g**2 + pp30 * omega_g_rpm**3 + 
                pp21 * omega_g_rpm**2 * tau_g + pp12 * omega_g_rpm * tau_g**2 + 
                pp03 * tau_g**3 + pp40 * omega_g_rpm**4 + 
                pp31 * omega_g_rpm**3 * tau_g + pp22 * omega_g_rpm**2 * tau_g**2 + 
                pp13 * omega_g_rpm * tau_g**3 + pp04 * tau_g**4)
        e_g = e_g_org * 0.01

        f_roll = mu * m * g
        f_drag = 0.5 * A * C_D * rho * (v ** 2)
        v_dot = ((r * G * tau_mg) - (r**2 * (f_roll + f_drag + f_brk))) / (G**2 * j_mg + j_w + m * r**2)
        P_gen = omega_g * tau_g * e_g / 1000
        P_mot = (m * v_dot + f_roll + f_drag + f_brk) * v / 1000
        e_m = 1
        P_c = e_m * P_mot

        P_bat = P_gen - P_c + C
        f1.append(P_bat)

    f1 = cs.vertcat(*f1)
    return [f1, states_bound]

states_0 = cs.MX.sym("states_0", NX)
controls_k = [cs.MX.sym('controls_' + str(i), NU) for i in range(Th)]

states_t = states_0
total_cost = 0
for k in range(0, Th):
    total_cost += imanishiLagrange_cost(states_t, controls_k[k])
    states_t = dynamic_dt(states_t, controls_k[k])

segment_ids = [i for i in range(NU*Th)]
Cu1 = og.constraints.Rectangle(xmin=[0], xmax=[103])
Cu2 = og.constraints.FiniteSet([[0], [1]])
Cu3 = og.constraints.FiniteSet([[0], [1]])
Cu4 = og.constraints.Rectangle(xmin=[-254], xmax=[254])
Cu_list = [Cu1, Cu2, Cu3, Cu4]*Th

input_bound = og.constraints.CartesianProduct(segment_ids, Cu_list)

[f1,Cs] = state_constraint(states_0, controls_k)
f2 = penaltyConstraints(states_0, controls_k)

optimization_variables = []
optimization_parameters = []

optimization_variables += controls_k
optimization_parameters += [states_0]
optimization_variables = cs.vertcat(*optimization_variables)
optimization_parameters = cs.vertcat(*optimization_parameters)

problem = og.builder.Problem(optimization_variables,
                             optimization_parameters,
                             total_cost) \
    .with_aug_lagrangian_constraints(f1, Cs) \
    .with_penalty_constraints(f2) \
    .with_constraints(input_bound)

build_config = og.config.BuildConfiguration()  \
    .with_build_directory("python_build")      \
    .with_build_mode("release")                \
    .with_tcp_interface_config()

meta = og.config.OptimizerMeta().with_optimizer_name("guan_optimizer")

solver_config = og.config.SolverConfiguration()\
    .with_tolerance(5000)\
    .with_initial_tolerance(5000)\
    .with_initial_penalty(1e-1)

builder = og.builder.OpEnOptimizerBuilder(problem, meta,
                                          build_config, solver_config)
builder.build()