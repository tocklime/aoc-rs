import sympy

# My input: 
pos1, v1 = ([315268300752660,284016300325583,407533418983227], [-11,23,-52])
pos2, v2 = ([393927681060873,429508206398995,348027409734393], [-239,-271,-115])
pos3, v3 = ([279975598233486,285305766984543,322446398749056], [-37,-9,-83])

# Example:
# pos1, v1 = ([19,13,30], [-2,1,-2])
# pos2, v2 = ([18,19,22], [-1,-1,-2])
# pos3, v3 = ([20,25,34], [-2,-2,-4])

posx, posy, posz, vx, vy, vz, t1, t2, t3 = sympy.symbols("posx posy posz vx vy vz t1 t2 t3", real=True)
t = [t1, t2, t3]
pos = [posx, posy, posz]
v = [vx, vy, vz]
input_pos = [pos1, pos2, pos3]
input_vel = [v1, v2, v3]
equations = []
for t_ix in range(3):
    for d_ix in range(3):
        equations.append(sympy.Eq(pos[d_ix] + v[d_ix] * t[t_ix], input_pos[t_ix][d_ix] + input_vel[t_ix][d_ix] * t[t_ix]))

solution = sympy.solve(equations)[0]  
vals = list(solution.values())
print(vals[0] + vals[1] + vals[2])

