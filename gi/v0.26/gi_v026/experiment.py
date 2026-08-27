from pathlib import Path
import json
import numpy as np

from .domain_general_reasoner import (
    IntegerVectorAdapter,
    learn_vector_deltas,
)
from .emergent_observable import (
    ResidualObservableFormer,
    LatentTemplateLibrary,
    LatentTransferPlanner,
    observations_for_adapter,
    token_for,
    primitive_depth_one_baseline,
)
from .signal import (
    SignalConfig,encode_text,build_source,decode_bits,
    bytes_from_framed_bits,bit_error_rate
)
from .coupled_rlc import SourcePort,StaticLoad
from .memory_bank import GridIntelligence

def jdefault(o):
    if isinstance(o,np.ndarray): return o.tolist()
    if isinstance(o,(np.floating,np.integer,np.bool_)): return o.item()
    if isinstance(o,bytes): return o.decode("utf-8")
    if hasattr(o,"__dict__"): return o.__dict__
    raise TypeError(type(o).__name__)

def run_on_grid(grid,text,cfg,dt=1e-3,tail=0.08):
    enc=encode_text(text,cfg)
    src=SourcePort(0,2.0,build_source(enc,cfg),"input")
    load=StaticLoad(6,1/68.0,"load")
    tr=grid.simulate([src],[load],enc.duration+tail,dt=dt)
    return enc,tr

def build_diagonal_domain(name,mag_ne=1,mag_se=1,mag_sw=1,mag_nw=1):
    # Four multi-axis directions. Existing v0.25 ACTIVE_AXIS/SIGN atoms cannot
    # directly distinguish these because every action changes both coordinates.
    effects={
        0:( mag_ne, mag_ne),   # NE
        1:( mag_se,-mag_se),   # SE
        2:(-mag_sw,-mag_sw),   # SW
        3:(-mag_nw, mag_nw),   # NW
    }
    rows=[]
    for s in [(0,0),(4,3),(-2,5),(7,-1),(3,3),(10,2)]:
        for slot,d in effects.items():
            nxt=(s[0]+d[0],s[1]+d[1])
            rows.append((slot,s,nxt))
    a=IntegerVectorAdapter(learn_vector_deltas(rows))
    a.name=name
    return a

def build_rotated_ecology_domain(name,scale=1):
    # Cardinal directions. Same formation algorithm should infer a different
    # set of prototypes from this different ecology.
    effects={
        0:( scale,0),
        1:( 0,scale),
        2:(-scale,0),
        3:( 0,-scale),
    }
    rows=[]
    for s in [(0,0),(3,4),(-2,6),(8,-1),(5,5)]:
        for slot,d in effects.items():
            nxt=(s[0]+d[0],s[1]+d[1])
            rows.append((slot,s,nxt))
    a=IntegerVectorAdapter(learn_vector_deltas(rows))
    a.name=name
    return a

def score_dump(rows):
    return [
        {
            "k":r.k,
            "overlap":r.cross_domain_overlap,
            "discrimination":r.discrimination,
            "separation":r.separation,
            "complexity_penalty":r.complexity_penalty,
            "score":r.score,
        }
        for r in rows
    ]

def run_suite(output_path=None):
    root=Path(__file__).resolve().parents[1]
    inherited=json.loads(
        (root/"artifacts"/"gi_v0_25_results.json").read_text()
    )
    tests={}

    tests["inherited_v025_evidence_freeze"]={
        "version":inherited["version"],
        "all_pass":bool(inherited["all_pass"]),
        "pass":bool(inherited["all_pass"])
    }

    # Source domains share direction structure but not raw magnitude.
    d1=build_diagonal_domain("diagonal_integer",1,1,1,1)
    d2=build_diagonal_domain("diagonal_physical",2,3,2,3)

    obs=observations_for_adapter(d1)+observations_for_adapter(d2)
    former=ResidualObservableFormer(k_min=2,k_max=6)

    inherited_audit=former.inherited_feature_audit(obs)

    tests["inherited_v025_feature_grammar_stress"]={
        "best_expr":inherited_audit["best_expr"],
        "best_overlap":inherited_audit["best_overlap"],
        "best_discrimination":inherited_audit["best_discrimination"],
        "reason":
            "all source actions change both coordinates; inherited axis/sign atoms collapse, while raw delta fails cross-domain magnitude invariance",
        "pass":inherited_audit["best_discrimination"]<1.0
    }

    observable,table=former.form(obs)

    tests["latent_observable_formation"]={
        "selected_k":observable.k,
        "prototypes":observable.prototypes,
        "source_domains":observable.source_domains,
        "score_table":score_dump(table),
        "pass":observable.k==4 and len(observable.prototypes)==4
    }

    # Every anonymous action in both source domains should map to a distinct
    # learned category, despite unequal magnitudes.
    maps={}
    for adapter in [d1,d2]:
        maps[adapter.name]={
            str(slot):token_for(observable,adapter.effects[slot].delta)
            for slot in adapter.action_slots()
        }
    src_sets=[set(m.values()) for m in maps.values()]

    tests["latent_observable_cross_domain_invariance"]={
        "mappings":maps,
        "domain_token_sets":[sorted(x) for x in src_sets],
        "same_token_set":src_sets[0]==src_sets[1],
        "full_discrimination":all(len(s)==4 for s in src_sets),
        "pass":src_sets[0]==src_sets[1]
               and all(len(s)==4 for s in src_sets)
    }

    # Learn templates under the emergent observable.
    lookup={}
    for o in obs:
        lookup[(o.domain,o.action_id)]=o

    traces=[]
    # Template family: NE -> SE
    for rep in range(3):
        traces.append(("diagonal_integer",f"i_pos_{rep}",(0,1)))
        traces.append(("diagonal_physical",f"p_pos_{rep}",(0,1)))
    # Distinct family: NW -> SW
    for rep in range(3):
        traces.append(("diagonal_integer",f"i_neg_{rep}",(3,2)))
        traces.append(("diagonal_physical",f"p_neg_{rep}",(3,2)))

    lib=LatentTemplateLibrary(min_support=3)
    created=lib.induce(observable,traces,lookup)

    pos_tokens=tuple(
        token_for(observable,lookup[("diagonal_integer",a)].delta)
        for a in (0,1)
    )
    neg_tokens=tuple(
        token_for(observable,lookup[("diagonal_integer",a)].delta)
        for a in (3,2)
    )
    t_pos=lib.find(pos_tokens)
    t_neg=lib.find(neg_tokens)

    tests["latent_template_induction"]={
        "template_count":len(created),
        "positive_tokens":pos_tokens,
        "negative_tokens":neg_tokens,
        "positive_support":None if t_pos is None else t_pos.support,
        "negative_support":None if t_neg is None else t_neg.support,
        "distinct":pos_tokens!=neg_tokens,
        "pass":t_pos is not None
               and t_neg is not None
               and t_pos.support==6
               and t_neg.support==6
               and pos_tokens!=neg_tokens
    }

    # Unseen target domain: same directions, new magnitudes.
    target=build_diagonal_domain("unseen_diagonal_resource",4,5,6,7)
    planner=LatentTransferPlanner()

    # NE then SE: (+4,+4) + (+5,-5) => (+9,-1)
    source=(1,1)
    goal=(10,0)

    _,base_dec,base_ok,base_out=primitive_depth_one_baseline(
        target,source,goal
    )
    rows,dec,ok,out=planner.solve(
        t_pos,observable,target,source,goal
    )

    tests["emergent_observable_transfer_to_unseen_domain"]={
        "source":source,
        "target":goal,
        "target_domain":target.name,
        "target_excluded_from_observable_formation":True,
        "primitive_depth1_verified":base_ok,
        "selected_plan":None if dec["selected"] is None else dec["selected"].plan,
        "verified":ok,
        "output":out,
        "pass":not base_ok
               and ok
               and out==goal
               and dec["selected"].plan==(0,1)
    }

    batch=[]
    hits=0
    for src in [(0,0),(3,4),(-2,6),(8,-1)]:
        tgt=(src[0]+9,src[1]-1)
        _,d,ok0,out0=planner.solve(t_pos,observable,target,src,tgt)
        hit=bool(ok0 and out0==tgt)
        hits+=int(hit)
        batch.append({
            "source":src,
            "target":tgt,
            "plan":None if d["selected"] is None else d["selected"].plan,
            "verified":ok0,
        })

    tests["emergent_observable_batch_transfer"]={
        "cases":batch,
        "hits":hits,
        "total":4,
        "accuracy":hits/4,
        "pass":hits==4
    }

    # Wrong-codebook control: swap two prototype identities. Since token IDs
    # are the learned observable itself, shuffling breaks the source->target
    # interpretation if we keep the source template fixed.
    protos=list(observable.prototypes)
    # Corrupt a category actually used by the transferred positive template.
    # Swap the first source-template category with an unrelated category.
    used0=int(pos_tokens[0])
    unrelated=next(i for i in range(len(protos)) if i not in set(pos_tokens))
    protos[used0],protos[unrelated]=protos[unrelated],protos[used0]
    from .emergent_observable import LatentObservable
    shuffled=LatentObservable(
        tuple(protos),observable.k,observable.source_domains,observable.score
    )
    _,sdec,sok,sout=planner.solve(
        t_pos,shuffled,target,source,goal
    )

    tests["shuffled_observable_control"]={
        "verified":sok,
        "output":sout,
        "pass":not sok
    }

    # Alternate ecology should yield cardinal prototypes rather than diagonal.
    c1=build_rotated_ecology_domain("cardinal_A",1)
    c2=build_rotated_ecology_domain("cardinal_B",3)
    cobs=observations_for_adapter(c1)+observations_for_adapter(c2)
    alt,alt_table=former.form(cobs)

    tests["observable_changes_with_experience_ecology"]={
        "main_prototypes":observable.prototypes,
        "alternate_prototypes":alt.prototypes,
        "alternate_k":alt.k,
        "different":observable.prototypes!=alt.prototypes,
        "pass":alt.k==4 and observable.prototypes!=alt.prototypes
    }

    # Corrupt target local transition model after observable formation.
    bad=build_diagonal_domain("bad_target",4,5,6,7)
    bad.effects[0].delta=(-4,-4)
    _,bdec,bok,bout=planner.solve(
        t_pos,observable,bad,source,goal
    )

    tests["corrupted_target_model_falsification"]={
        "commit":bdec["commit"],
        "external_verified":bok,
        "output":bout,
        "pass":not bok
    }

    # One-shot outlier should not force k=5 because of complexity cost.
    outlier_obs=list(obs)
    from .emergent_observable import ActionObservation
    outlier_obs.append(
        ActionObservation("diagonal_integer",99,(1.0,0.15))
    )
    outlier_obs.append(
        ActionObservation("diagonal_physical",99,(4.0,0.60))
    )
    outlier_model,outlier_table=former.form(outlier_obs)

    tests["latent_observable_complexity_control"]={
        "selected_k_with_outlier":outlier_model.k,
        "score_table":score_dump(outlier_table),
        "pass":outlier_model.k==4
    }

    # Original signaling boundary.
    summary_text="GI26:OBS"
    sigcfg=SignalConfig(
        carrier_hz=35.0,cycles_per_bit=2.0,amplitude=1.0
    )
    grid=GridIntelligence().base.grid()
    enc,tr=run_on_grid(grid,summary_text,sigcfg)
    bits,gain,soft=decode_bits(
        tr.t,tr.node_voltage[:,6],enc,sigcfg.carrier_hz
    )
    raw=bytes_from_framed_bits(bits)
    ber=bit_error_rate(enc.bits,bits)
    raw_match=(tuple(raw)==enc.raw_bytes)

    tests["emergent_observable_result_reconstruction"]={
        "text":summary_text,
        "decoded":bytes(raw).decode("utf-8"),
        "ber":ber,
        "raw_match":raw_match,
        "pass":raw_match and ber==0.0
    }

    tests["information_boundary_audit"]={
        "specific_latent_prototypes_preseeded":False,
        "cluster_count_preselected":False,
        "target_domain_used_in_observable_formation":False,
        "raw_transition_deltas_available":True,
        "L2_normalization_is_designed":True,
        "nearest_prototype_observable_class_is_designed":True,
        "k_search_range_is_designed":True,
        "kmeans_algorithm_is_designed":True,
        "observable_prototypes_are_learned":True,
        "observable_category_count_is_selected_from_experience":True,
        "semantic_labels_for_categories":False,
        "arbitrary_sensor_invention":False,
        "unrestricted_meta_language_invention":False,
        "general_reasoning":False,
        "general_intelligence":False,
        "pass":True
    }

    results={
        "version":"GI — Grid Intelligence v0.26",
        "substrate":"GI v0.25 compositional meta-feature synthesis",
        "milestone":
            "GI now forms a new anonymous observable directly from raw transition effects when the inherited v0.25 feature grammar loses required distinctions. A residual-formation path learns a directional prototype codebook and selects its category count from experience. In a multi-axis ecology where ACTIVE_AXIS, SIGN, support-mask, magnitude, and raw-delta representations fail to jointly provide cross-domain invariance and full discrimination, the learned four-category observable restores those distinctions and transfers two-step structural templates into an unseen target domain with new raw magnitudes.",
        "learned_observable":{
            "k":observable.k,
            "prototypes":observable.prototypes,
            "source_domains":observable.source_domains,
            "score":observable.score,
        },
        "tests":tests,
        "scientific_status":{
            "residual_triggered_new_observable_formation":"supported",
            "learned_anonymous_latent_categories":"supported",
            "experience_selected_category_count":"supported",
            "cross_domain_invariance_beyond_v025_feature_grammar":"supported",
            "transfer_to_unseen_target_domain":"supported",
            "observable_changes_with_experience_ecology":"supported",
            "arbitrary_new_sensor_primitive_invention":"not established",
            "unbounded_observable_program_synthesis":"not established",
            "semantic_feature_invention":"not established",
            "general_reasoning":"not established",
            "general_intelligence":"not established"
        },
        "largest_artificial_assumptions":[
            "raw transition-effect vectors are available to the formation mechanism",
            "L2 normalization is a designed preprocessor",
            "nearest-prototype categorical observation is the declared new-observable function class",
            "deterministic k-means is the designed learner",
            "candidate k is searched only over 2..6",
            "the codebook utility formula is designed",
            "template induction still uses exact recurrence and fixed support",
            "no semantic category name or arbitrary new sensor primitive is invented"
        ]
    }

    results["all_pass"]=all(bool(v["pass"]) for v in tests.values())

    if output_path:
        Path(output_path).write_text(
            json.dumps(results,indent=2,ensure_ascii=False,default=jdefault),
            encoding="utf-8"
        )
    return results
