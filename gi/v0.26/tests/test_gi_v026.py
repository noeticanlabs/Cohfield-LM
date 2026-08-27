from gi_v026.experiment import run_suite

def test_gi_v026():
    r=run_suite()
    assert r["all_pass"]
    assert r["tests"]["inherited_v025_feature_grammar_stress"]["pass"]
    assert r["tests"]["latent_observable_formation"]["selected_k"] == 4
    assert r["tests"]["latent_observable_cross_domain_invariance"]["full_discrimination"]
    assert r["tests"]["emergent_observable_transfer_to_unseen_domain"]["verified"]
    assert r["tests"]["emergent_observable_batch_transfer"]["accuracy"] == 1.0
    assert r["tests"]["shuffled_observable_control"]["pass"]
    assert r["tests"]["observable_changes_with_experience_ecology"]["pass"]
    assert r["tests"]["corrupted_target_model_falsification"]["pass"]
    assert r["tests"]["latent_observable_complexity_control"]["pass"]
    assert r["scientific_status"]["general_intelligence"] == "not established"
