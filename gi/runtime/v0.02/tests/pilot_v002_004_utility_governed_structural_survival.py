import numpy as np, json, copy
from dataclasses import dataclass
from pathlib import Path

@dataclass
class UtilityPolicy:
    horizon: int = 240
    dt: float = 0.025
    epsilon: float = 1e-4
    complexity_cost: float = 0.01

class ToyAdaptiveGrid:
    """
    Minimal identity-preserving structural testbed for Pilot 004.

    State:
      X_i : fast scalar activity
      primitive_ids : persistent identity
      W[j,i] : directed coupling i -> j
      u_i : external forcing

    Dynamics:
      dX/dt = u + W X - decay*X - nonlinear*X^3

    Birth is a proposal, not automatically persistent.
    """
    def __init__(self, X, W, u, decay=0.55, nonlinear=0.045):
        self.X = np.asarray(X, dtype=float).copy()
        self.W = np.asarray(W, dtype=float).copy()
        self.u = np.asarray(u, dtype=float).copy()
        self.decay = float(decay)
        self.nonlinear = float(nonlinear)
        self.primitive_ids = list(range(len(self.X)))
        self.next_id = len(self.X)
        self.events = []

    @property
    def N(self): return len(self.X)

    def clone(self):
        return copy.deepcopy(self)

    def step(self, dt):
        d = self.u + self.W @ self.X - self.decay*self.X - self.nonlinear*(self.X**3)
        self.X = self.X + dt*d
        return self.X.copy()

    def trajectory(self, horizon, dt):
        tr = [self.X.copy()]
        for _ in range(horizon):
            tr.append(self.step(dt))
        return np.asarray(tr)

    def birth_split(self, source_id, mode="share"):
        """
        Birth from an existing stressed primitive.

        share:
          split parent's state 50/50 and mirror its outgoing/incoming
          couplings at half strength so the newborn can share future burden
          while approximately conserving aggregate coupling.

        isolated:
          split state but create no couplings. This is a neutral/weak control.

        harmful:
          mirror couplings at amplified gain, intentionally testing whether
          utility governance rejects destabilizing growth.
        """
        src = self.primitive_ids.index(source_id)
        oldN = self.N
        new_id = self.next_id
        self.next_id += 1

        x_new = 0.5*self.X[src]
        self.X[src] *= 0.5
        self.X = np.r_[self.X, x_new]
        source_u = float(self.u[src])
        self.u = np.r_[self.u, 0.0]

        Wn = np.zeros((oldN+1, oldN+1), dtype=float)
        Wn[:oldN,:oldN] = self.W

        if mode == "share":
            self.u[src] = 0.5*source_u
            self.u[oldN] = 0.5*source_u
            old_col = self.W[:,src].copy()
            old_row = self.W[src,:].copy()
            Wn[:oldN,src] = 0.5*old_col
            Wn[:oldN,oldN] = 0.5*old_col
            Wn[src,:oldN] = 0.5*old_row
            Wn[oldN,:oldN] = 0.5*old_row
        elif mode == "isolated":
            pass
        elif mode == "harmful":
            self.u[src] = 0.5*source_u
            self.u[oldN] = 0.5*source_u
            old_col = self.W[:,src].copy()
            old_row = self.W[src,:].copy()
            Wn[:oldN,oldN] = 1.5*old_col
            Wn[oldN,:oldN] = 1.5*old_row
            Wn[oldN,src] = 0.45
            Wn[src,oldN] = 0.45
        else:
            raise ValueError(mode)

        self.W = Wn
        self.primitive_ids.append(new_id)
        self.events.append({
            "event":"BIRTH_PROPOSAL",
            "source_id":int(source_id),
            "new_id":int(new_id),
            "mode":mode,
            "old_N":oldN,
            "new_N":oldN+1
        })
        return new_id

    def retire(self, pid, reason):
        idx = self.primitive_ids.index(pid)
        self.X = np.delete(self.X, idx)
        self.u = np.delete(self.u, idx)
        self.W = np.delete(np.delete(self.W, idx, axis=0), idx, axis=1)
        self.primitive_ids.pop(idx)
        self.events.append({"event":"RETIRE","primitive_id":int(pid),"reason":reason,"new_N":self.N})

def native_burden(traj):
    """
    Native structural burden:
      0.60 * peak concentration
    + 0.30 * mean quadratic activity
    + 0.10 * temporal roughness

    Lower is better. No semantic/task label appears in the metric.
    """
    a = np.abs(traj)
    total = np.mean(a, axis=1) + 1e-12
    concentration = np.max(a, axis=1)/total
    peak_conc = float(np.mean(concentration))
    energy = float(np.mean(traj**2))
    rough = float(np.mean(np.abs(np.diff(traj, axis=0))))
    return 0.60*peak_conc + 0.30*energy + 0.10*rough, {
        "mean_concentration":peak_conc,
        "mean_energy":energy,
        "mean_roughness":rough,
    }

def evaluate_birth(base, source_id, mode, policy):
    control = base.clone()
    trial = base.clone()

    control_traj = control.trajectory(policy.horizon, policy.dt)
    J0, m0 = native_burden(control_traj)

    newborn = trial.birth_split(source_id, mode)
    trial_traj = trial.trajectory(policy.horizon, policy.dt)
    J1raw, m1 = native_burden(trial_traj)
    J1 = J1raw + policy.complexity_cost*(trial.N - base.N)

    gain = J0 - J1
    commit = gain > policy.epsilon
    if commit:
        trial.events.append({
            "event":"COMMIT",
            "primitive_id":int(newborn),
            "utility_gain":float(gain),
            "baseline_cost":float(J0),
            "trial_cost":float(J1)
        })
    else:
        trial.retire(newborn, "UTILITY_REJECT")
        trial.events.append({
            "event":"REJECT",
            "primitive_id":int(newborn),
            "utility_gain":float(gain),
            "baseline_cost":float(J0),
            "trial_cost":float(J1)
        })

    return {
        "commit":commit,
        "gain":float(gain),
        "baseline_cost":float(J0),
        "trial_cost":float(J1),
        "baseline_metrics":m0,
        "trial_metrics":m1,
        "newborn":int(newborn),
        "final_ids":list(trial.primitive_ids),
        "events":trial.events,
        "final_state":trial.X.tolist(),
    }

def make_hotspot():
    W = np.array([
        [0.0, 0.10, 0.00, 0.04],
        [0.22,0.0,  0.06, 0.00],
        [0.00,0.16, 0.0,  0.08],
        [0.10,0.00, 0.12, 0.0],
    ])
    X = [2.8, 0.55, 0.45, 0.50]
    u = [1.40, 0.08, 0.05, 0.05]
    return ToyAdaptiveGrid(X,W,u)

R={}
def rec(name, ok, **metrics):
    R[name]={"pass":bool(ok), **metrics}

policy=UtilityPolicy()

useful=evaluate_birth(make_hotspot(),0,"share",policy)
rec("useful_birth_committed",
    useful["commit"] and useful["gain"]>policy.epsilon and len(useful["final_ids"])==5,
    **useful)

neutral=evaluate_birth(make_hotspot(),0,"isolated",policy)
rec("neutral_birth_rejected",
    (not neutral["commit"]) and len(neutral["final_ids"])==4,
    **neutral)

harmful=evaluate_birth(make_hotspot(),0,"harmful",policy)
rec("harmful_birth_rejected",
    (not harmful["commit"]) and harmful["gain"]<=policy.epsilon and len(harmful["final_ids"])==4,
    **harmful)

uniform = make_hotspot()
uniform.X[:] = np.mean(uniform.X)
uniform.u[:] = np.mean(uniform.u)
uniform_case=evaluate_birth(uniform,0,"share",policy)
rec("utility_depends_on_structural_pressure",
    useful["gain"] > uniform_case["gain"],
    hotspot_gain=useful["gain"], uniform_gain=uniform_case["gain"],
    uniform_commit=uniform_case["commit"])

cheap=UtilityPolicy(complexity_cost=0.0)
costly=UtilityPolicy(complexity_cost=0.04)
a=evaluate_birth(make_hotspot(),0,"share",cheap)
b=evaluate_birth(make_hotspot(),0,"share",costly)
rec("complexity_cost_opposes_unbounded_growth",
    a["gain"] > b["gain"],
    zero_penalty_gain=a["gain"], high_penalty_gain=b["gain"])

wrong=evaluate_birth(make_hotspot(),2,"share",policy)
rec("utility_selects_structurally_relevant_birth",
    useful["gain"] > wrong["gain"],
    stressed_source_gain=useful["gain"], wrong_source_gain=wrong["gain"],
    wrong_source_commit=wrong["commit"])

u2=evaluate_birth(make_hotspot(),0,"share",policy)
rec("deterministic_utility_replay",
    json.dumps(useful,sort_keys=True)==json.dumps(u2,sort_keys=True),
    byte_equivalent=json.dumps(useful,sort_keys=True)==json.dumps(u2,sort_keys=True))

ev=[e["event"] for e in useful["events"]]
rec("proposal_commit_separation",
    ev[0]=="BIRTH_PROPOSAL" and "COMMIT" in ev,
    event_sequence=ev)

R["summary"]={
    "passed":sum(v["pass"] for v in R.values()),
    "total":len(R),
}
R["summary"]["overall_pass"]=R["summary"]["passed"]==R["summary"]["total"]
R["summary"]["pilot_status"]="UTILITY_GOVERNED_STRUCTURAL_SURVIVAL_PRESENT" if R["summary"]["overall_pass"] else "REPAIR_REQUIRED"
Path("GI_Runtime_Kernel_v0.02_Pilot_004_results.json").write_text(json.dumps(R,indent=2))
print(json.dumps(R,indent=2))
