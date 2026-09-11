"""Exact opaque-control assertions across every actual diagnostic channel."""
import re

CONVERGENCE_MESSAGES = (
    re.compile(r'^document did not converge within five attempts$'),
    re.compile(r'^value of `.*` did not converge$'),
)

def assert_opaque(observed):
    assert observed['body']['result']=='Ok'
    assert observed['has_filtered_counter_reads'] is True
    # The actual relation DTO can be passed as the exact private discriminant;
    # it may not be derived from public false or an error.
    assert observed['private_discriminant']=='Unproven'
    assert set(observed['validation'])=={'independent'}
    assert set(observed['validation_sinks'])=={'independent'}
    channels=[('construction_sink',observed['construction_sink']),
              ('body_sink',observed['body_sink']),
              ('diagnosis_sink',observed['diagnosis_sink']),
              ('validation_sinks.independent',observed['validation_sinks']['independent'])]
    row=observed['validation']['independent']
    if row['result']=='Ok':
        assert row['value'] is False
    else:
        assert row['result']=='Err' and isinstance(row['diagnostics'],list) and row['diagnostics']
        channels.append(('validation.independent.Err',row['diagnostics']))
    row=observed['nonconvergence_diagnostics']
    if row['result']=='Ok':
        assert row['length']==0 and row['diagnostics']==[]
    else:
        assert row['result']=='Err' and isinstance(row['diagnostics'],list) and row['diagnostics']
        channels.append(('nonconvergence_diagnostics.Err',row['diagnostics']))
    for channel,diagnostics in channels:
        assert isinstance(diagnostics,list),channel
        for diagnostic in diagnostics:
            assert diagnostic['severity']=='Error',(channel,'opaque result cannot emit warnings')
            assert not any(p.fullmatch(diagnostic['message']) for p in CONVERGENCE_MESSAGES),(channel,'nonconvergence message cannot be disguised as Err')
    # These sources contain no ordinary warning/error side effects outside
    # the deliberately opaque validator operation.
    assert observed['construction_sink']==[]
    assert observed['body_sink']==[]
    return True
