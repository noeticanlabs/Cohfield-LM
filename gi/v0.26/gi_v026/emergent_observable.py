from __future__ import annotations
from dataclasses import dataclass
from collections import defaultdict
from typing import Any
import math
import itertools

from .domain_general_reasoner import DomainGeneralReasoner, DGConfig
from .meta_feature_synthesis import (
    ActionObservation,
    FeatureExpr,
    MetaFeatureSynthesizer,
    expression_basis,
    expr_value,
)

@dataclass(frozen=True)
class LatentObservable:
    """Anonymous learned codebook over normalized raw transition effects."""
    prototypes: tuple[tuple[float,...],...]
    k:int
    source_domains: tuple[str,...]
    score:float

@dataclass(frozen=True)
class CodebookScore:
    k:int
    cross_domain_overlap:float
    discrimination:float
    separation:float
    complexity_penalty:float
    score:float

@dataclass
class LatentTemplate:
    slot:int
    tokens:tuple[int,...]
    support:int
    source_domains:tuple[str,...]

@dataclass
class PlanCandidate:
    template_slot:int
    plan:tuple[int,...]
    predicted_state:Any
    similarity:float
    confidence:float
    score:float


def _l2(v):
    return math.sqrt(sum(float(x)*float(x) for x in v))

def normalize_delta(delta):
    n=_l2(delta)
    if n<=1e-15:
        return tuple(0.0 for _ in delta)
    return tuple(float(x)/n for x in delta)

def squared_distance(a,b):
    return sum((float(x)-float(y))**2 for x,y in zip(a,b))

def canonicalize_prototypes(prototypes):
    return tuple(sorted(
        (tuple(float(x) for x in p) for p in prototypes),
        key=lambda p: tuple(round(x,12) for x in p)
    ))

def nearest(prototypes, vector):
    ds=[squared_distance(vector,p) for p in prototypes]
    return min(range(len(ds)), key=lambda i:(ds[i],i))

def _unique_vectors(vectors):
    out=[]
    seen=set()
    for v in vectors:
        key=tuple(round(float(x),12) for x in v)
        if key not in seen:
            seen.add(key)
            out.append(tuple(float(x) for x in v))
    return out

def _mean(vs):
    if not vs:
        raise ValueError("empty cluster")
    d=len(vs[0])
    return tuple(sum(v[i] for v in vs)/len(vs) for i in range(d))

def deterministic_kmeans(vectors,k,max_iter=64):
    """Deterministic k-means with farthest-first data-derived seeding."""
    uniq=_unique_vectors(vectors)
    if k<1 or k>len(uniq):
        return None

    # First centroid: lexicographically smallest observed normalized effect.
    centers=[min(uniq,key=lambda v:tuple(round(x,12) for x in v))]

    # Remaining centroids: observed point farthest from current set.
    while len(centers)<k:
        candidates=[]
        for v in uniq:
            if v in centers:
                continue
            d=min(squared_distance(v,c) for c in centers)
            candidates.append((d,tuple(round(x,12) for x in v),v))
        candidates.sort(key=lambda x:(-x[0],x[1]))
        centers.append(candidates[0][2])

    centers=list(canonicalize_prototypes(centers))

    for _ in range(max_iter):
        groups=[[] for _ in range(k)]
        for v in vectors:
            groups[nearest(centers,v)].append(v)
        if any(not g for g in groups):
            return None
        new=list(canonicalize_prototypes([_mean(g) for g in groups]))
        if all(squared_distance(a,b)<=1e-24 for a,b in zip(centers,new)):
            break
        centers=new

    return tuple(centers)


def pairwise_jaccard(sets):
    if len(sets)<2:
        return 0.0
    vals=[]
    for i in range(len(sets)):
        for j in range(i+1,len(sets)):
            a,b=sets[i],sets[j]
            u=a|b
            vals.append(1.0 if not u else len(a&b)/len(u))
    return sum(vals)/len(vals)


class ResidualObservableFormer:
    """Construct a new observable when inherited feature grammar is inadequate.

    The inherited v0.25 feature grammar is audited first. If no inherited
    expression provides high discrimination and cross-domain invariance, the
    system learns a nearest-prototype codebook directly from normalized raw
    transition effects.

    The specific prototypes and selected cluster count are learned from data.
    """

    def __init__(self,k_min=2,k_max=6):
        self.k_min=int(k_min)
        self.k_max=int(k_max)

    def inherited_feature_audit(self,observations):
        synth=MetaFeatureSynthesizer()
        exprs=expression_basis(max_depth=2)
        best,table=synth.synthesize(observations,exprs)
        top=table[0]
        return {
            "best_expr":str(best),
            "best_score":float(top.score),
            "best_overlap":float(top.overlap),
            "best_discrimination":float(top.discrimination),
            "table":table,
        }

    def _score_codebook(self,observations,prototypes):
        by_domain=defaultdict(list)
        for o in observations:
            tok=nearest(prototypes,normalize_delta(o.delta))
            by_domain[o.domain].append((o.action_id,tok))

        domain_sets=[]
        discr=[]
        for domain,rows in sorted(by_domain.items()):
            token_set={tok for _,tok in rows}
            domain_sets.append(token_set)
            n_actions=len({aid for aid,_ in rows})
            discr.append(len(token_set)/max(1,n_actions))

        overlap=pairwise_jaccard(domain_sets)
        discrimination=sum(discr)/max(1,len(discr))

        # Geometric separation among prototypes.
        if len(prototypes)<=1:
            separation=0.0
        else:
            dists=[
                math.sqrt(squared_distance(a,b))
                for i,a in enumerate(prototypes)
                for b in prototypes[i+1:]
            ]
            # normalized directional vectors live in a ball of diameter 2.
            separation=min(dists)/2.0

        complexity=0.075*len(prototypes)
        score=1.50*overlap + discrimination + 0.50*separation - complexity
        return CodebookScore(
            len(prototypes),
            float(overlap),
            float(discrimination),
            float(separation),
            float(complexity),
            float(score),
        )

    def form(self,observations):
        vectors=[normalize_delta(o.delta) for o in observations]
        uniq=_unique_vectors(vectors)
        rows=[]
        models=[]
        hi=min(self.k_max,len(uniq))
        for k in range(self.k_min,hi+1):
            protos=deterministic_kmeans(vectors,k)
            if protos is None:
                continue
            s=self._score_codebook(observations,protos)
            rows.append(s)
            models.append((s,protos))

        if not models:
            raise ValueError("no viable latent observable")

        models.sort(
            key=lambda x:(-x[0].score,x[0].k,
                          tuple(tuple(round(v,12) for v in p) for p in x[1]))
        )
        best,protos=models[0]
        domains=tuple(sorted({o.domain for o in observations}))
        return LatentObservable(
            prototypes=tuple(protos),
            k=best.k,
            source_domains=domains,
            score=best.score,
        ), sorted(rows,key=lambda r:(-r.score,r.k))


def adapter_delta(adapter,slot):
    if not hasattr(adapter,"effects"):
        return None
    e=adapter.effects[int(slot)]
    if hasattr(e,"delta"):
        return tuple(float(x) for x in e.delta)
    if hasattr(e,"d_level") and hasattr(e,"d_temp"):
        return (float(e.d_level),float(e.d_temp))
    return None

def observations_for_adapter(adapter):
    rows=[]
    for slot in adapter.action_slots():
        d=adapter_delta(adapter,slot)
        if d is None:
            continue
        rows.append(ActionObservation(str(adapter.name),int(slot),tuple(d)))
    return rows

def token_for(observable,delta):
    return int(nearest(observable.prototypes,normalize_delta(delta)))


class LatentTemplateLibrary:
    def __init__(self,min_support=3):
        self.min_support=int(min_support)
        self.templates=[]

    def induce(self,observable,trace_rows,lookup):
        groups=defaultdict(list)
        domains=defaultdict(set)
        for domain,task_id,trace in trace_rows:
            toks=tuple(
                token_for(observable,lookup[(domain,int(a))].delta)
                for a in trace
            )
            groups[toks].append(task_id)
            domains[toks].add(domain)

        created=[]
        for toks,ids in groups.items():
            if len(set(ids))<self.min_support:
                continue
            t=LatentTemplate(
                slot=len(self.templates),
                tokens=toks,
                support=len(set(ids)),
                source_domains=tuple(sorted(domains[toks])),
            )
            self.templates.append(t)
            created.append(t)
        return created

    def find(self,tokens):
        tokens=tuple(tokens)
        for t in self.templates:
            if t.tokens==tokens:
                return t
        return None


class LatentTransferPlanner:
    def __init__(
        self,
        depth_cost=0.035,
        uncertainty_penalty=0.20,
        commit_score_threshold=0.84,
        commit_margin_threshold=0.05,
    ):
        self.depth_cost=float(depth_cost)
        self.uncertainty_penalty=float(uncertainty_penalty)
        self.commit_score_threshold=float(commit_score_threshold)
        self.commit_margin_threshold=float(commit_margin_threshold)

    def instantiate(self,template,observable,target_adapter):
        by_token=defaultdict(list)
        for slot in target_adapter.action_slots():
            d=adapter_delta(target_adapter,slot)
            if d is None:
                continue
            by_token[token_for(observable,d)].append(int(slot))

        choices=[]
        for tok in template.tokens:
            slots=by_token.get(tok,[])
            if not slots:
                return []
            choices.append(tuple(slots))

        plans=[tuple()]
        for slots in choices:
            plans=[p+(s,) for p in plans for s in slots]
        return plans

    def _rollout(self,adapter,source,plan):
        state=source
        confs=[]
        for slot in plan:
            state,c=adapter.predict_action(state,slot)
            if state is None:
                return None,0.0
            confs.append(float(c))
        return state,(min(confs) if confs else 1.0)

    def solve(self,template,observable,adapter,source,target):
        plans=self.instantiate(template,observable,adapter)
        rows=[]
        for plan in plans:
            pred,conf=self._rollout(adapter,source,plan)
            sim=adapter.similarity(pred,target) if pred is not None else 0.0
            score=sim-self.depth_cost-self.uncertainty_penalty*(1.0-conf)
            rows.append(
                PlanCandidate(
                    template.slot,tuple(plan),pred,float(sim),float(conf),float(score)
                )
            )

        rows.sort(key=lambda r:(-r.score,-r.similarity,-r.confidence,r.plan))
        if not rows:
            return rows,{"commit":False,"selected":None},False,None

        groups={}
        for r in rows:
            key=None if r.predicted_state is None else adapter.state_key(r.predicted_state)
            groups.setdefault(key,[]).append(r)

        reps=[]
        for key,members in groups.items():
            members=sorted(members,key=lambda r:(-r.score,r.plan))
            reps.append((members[0],members))
        reps.sort(key=lambda p:(-p[0].score,-p[0].similarity,-p[0].confidence,p[0].plan))

        top,eq=reps[0]
        second=reps[1][0] if len(reps)>1 else None
        margin=top.score-(second.score if second is not None else 0.0)
        commit=bool(
            top.score>=self.commit_score_threshold
            and margin>=self.commit_margin_threshold
        )
        dec={
            "commit":commit,
            "selected":top if commit else None,
            "score":float(top.score),
            "margin":float(margin),
            "distinct_future_count":len(reps),
        }

        if not commit:
            return rows,dec,False,None

        state=source
        for slot in top.plan:
            state=adapter.execute_action(state,slot)
            if state is None:
                break
        return rows,dec,bool(state==target),state


def primitive_depth_one_baseline(adapter,source,target):
    r=DomainGeneralReasoner(
        DGConfig(
            max_depth=1,
            depth_penalty=0.035,
            uncertainty_penalty=0.20,
            commit_score_threshold=0.84,
            commit_margin_threshold=0.05,
            top_k=64,
        )
    )
    return r.solve(adapter,source,target)
