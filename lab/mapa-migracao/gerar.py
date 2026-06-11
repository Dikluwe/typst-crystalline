#!/usr/bin/env python3
"""Gerador da parte MECÂNICA do Mapa de Migração vanilla <-> cristalino.

Arena (lab/). Fora das camadas L1-L4 e da trava de L0. Descartável e
regenerável: lê o JSON da lente (`lente --comparar`) e emite a tabela
mecânica em markdown no stdout. NUNCA editado à mão; a verdade mecânica
vem do JSON, não deste ficheiro.

A coluna DECLARADA (curada por humano/sessão, com fonte) NÃO vive aqui —
vive no documento `00_nucleo/mapa-migracao-vanilla-cristalino.md`. Este
script só conhece o que a lente mede.

Uso:
    python3 gerar.py /tmp/comparar-typst-itens.json > mecanica.md

As funções públicas (module_of, classify_module, build_modules) são
reutilizadas pelo passo de montagem do documento para garantir uma única
fonte da verdade na derivação de módulo e na sugestão mecânica.
"""
import json
import sys
from collections import defaultdict

# --- Thresholds (SUGESTÃO mecânica, NÃO veredito). Constantes nomeadas. ---
# >= MIGRADO_PCT de itens reais pareados  -> "migrado"
# 0 itens reais pareados                  -> "não-iniciado"
# 0 itens reais (só folhas de trait)      -> "só-boilerplate"
# entre 0 e MIGRADO_PCT                    -> "parcial"
MIGRADO_PCT = 0.90  # sugestão, não veredito humano

# --- Definição de BOILERPLATE -------------------------------------------
# DEFINIÇÃO PRIMÁRIA (medição 0077 da lente): item é boilerplate se o campo
# `trait` está preenchido (folha de impl-de-trait). Verificado: todas as
# listas do JSON (pareados, sem_par_antes, sem_par_depois, ambiguos)
# carregam o campo `trait`, logo o FALLBACK (fn-de-tipo com nome canónico)
# NÃO foi necessário e não é usado. Registado aqui e no README.
def is_boilerplate(item):
    return bool(item.get('trait'))


# --- Derivação do MÓDULO a partir do path -------------------------------
# Regras (registadas no README e no cabeçalho do documento):
#  1. Remover segmentos de resolução `<...>` (ex.: `Abs::<Add>::Output`).
#     Verificado no JSON: todo segmento de resolução, ao separar por `::`,
#     começa por `<` -> basta descartar segmentos que começam por `<`.
#  2. Remover o último segmento (o nome do item).
#  3. Enquanto o último segmento restante começar por maiúscula (tipos pai,
#     convenção Rust: módulos são snake_case, tipos são CamelCase), removê-lo.
#     O que sobra é o módulo. Mantém-se sempre >= 1 segmento (o crate).
def module_of(path):
    segs = [s for s in path.split('::') if not s.startswith('<')]
    if not segs:
        return None
    segs = segs[:-1]  # 2. nome do item
    while len(segs) > 1 and segs[-1][:1].isupper():  # 3. tipos pai
        segs.pop()
    return '::'.join(segs)


def crate_of(module):
    return module.split('::')[0]


# --- Construção da tabela por módulo do vanilla -------------------------
def build_modules(itens):
    """Devolve dict: módulo_vanilla -> métricas agregadas."""
    mods = defaultdict(lambda: {
        'reais_total': 0, 'bp_total': 0,
        'reais_pareados': 0, 'reais_sempar': 0,
        'destinos': defaultdict(int),  # módulo cristalino -> nº pareados reais
    })

    for x in itens['pareados']:
        m = module_of(x['de'])
        if m is None:
            continue
        rec = mods[m]
        if is_boilerplate(x):
            rec['bp_total'] += 1
        else:
            rec['reais_total'] += 1
            rec['reais_pareados'] += 1
            dest = module_of(x['para'])
            if dest is not None:
                rec['destinos'][dest] += 1

    for x in itens['sem_par_antes']:
        m = module_of(x['path'])
        if m is None:
            continue
        rec = mods[m]
        if is_boilerplate(x):
            rec['bp_total'] += 1
        else:
            rec['reais_total'] += 1
            rec['reais_sempar'] += 1

    return mods


def classify_module(rec):
    """Sugestão mecânica a partir das métricas. NÃO é veredito."""
    reais = rec['reais_total']
    if reais == 0:
        return 'só-boilerplate'
    if rec['reais_pareados'] == 0:
        return 'não-iniciado'
    if rec['reais_pareados'] / reais >= MIGRADO_PCT:
        return 'migrado'
    return 'parcial'


def dominant_dest(rec):
    """(módulo cristalino dominante, fração) sobre os pareados reais."""
    dst = rec['destinos']
    if not dst:
        return ('—', 0.0)
    total = sum(dst.values())
    # ordenar por contagem desc, depois por nome para determinismo
    best = sorted(dst.items(), key=lambda kv: (-kv[1], kv[0]))[0]
    return (best[0], best[1] / total)


def pct(num, den):
    return (num / den) if den else 0.0


# --- Censo inverso: módulos do cristalino com itens sem-par-depois -------
def build_cristalino_census(itens):
    census = defaultdict(lambda: {'reais': 0, 'bp': 0})
    for x in itens['sem_par_depois']:
        m = module_of(x['path'])
        if m is None:
            continue
        if is_boilerplate(x):
            census[m]['bp'] += 1
        else:
            census[m]['reais'] += 1
    return census


# --- Emissão markdown ----------------------------------------------------
def emit(data):
    itens = data['itens']
    mods = build_modules(itens)
    out = []
    w = out.append

    w('<!-- GERADO POR lab/mapa-migracao/gerar.py — NÃO EDITAR À MÃO. -->')
    w('<!-- Regenerar: python3 lab/mapa-migracao/gerar.py <json> > ... -->')
    w('')
    w(f"Limite de pareamento (lente): {data.get('limite_pareamento','?')}")
    w(f"Boilerplate = campo `trait` preenchido (def. primária 0077; "
      f"fallback não necessário).")
    w(f"Sugestão mecânica: migrado >= {int(MIGRADO_PCT*100)}% reais "
      f"pareados · parcial entre · não-iniciado 0 pareado · "
      f"só-boilerplate 0 reais.")
    w('')

    # Agrupar módulos por crate, ordenado.
    by_crate = defaultdict(list)
    for m in mods:
        by_crate[crate_of(m)].append(m)

    # ---- ROLLUP por crate (primeiro, visão de topo) ----
    w('## Rollup por crate (vanilla)')
    w('')
    w('| crate | módulos | migrados | parciais | não-iniciados | '
      'só-bp | reais pareados | reais sem-par |')
    w('|---|--:|--:|--:|--:|--:|--:|--:|')
    crate_real_par = defaultdict(int)
    crate_real_sp = defaultdict(int)
    for crate in sorted(by_crate):
        counts = defaultdict(int)
        for m in by_crate[crate]:
            counts[classify_module(mods[m])] += 1
            crate_real_par[crate] += mods[m]['reais_pareados']
            crate_real_sp[crate] += mods[m]['reais_sempar']
        w(f"| {crate} | {len(by_crate[crate])} | "
          f"{counts['migrado']} | {counts['parcial']} | "
          f"{counts['não-iniciado']} | {counts['só-boilerplate']} | "
          f"{crate_real_par[crate]} | {crate_real_sp[crate]} |")
    tot_par = sum(crate_real_par.values())
    tot_sp = sum(crate_real_sp.values())
    w(f"| **TOTAL** | {len(mods)} | | | | | {tot_par} | {tot_sp} |")
    w('')

    # ---- Tabela por módulo, por crate ----
    w('## Mecânica por módulo do vanilla')
    w('')
    for crate in sorted(by_crate):
        w(f'### {crate}')
        w('')
        w('| módulo | itens (reais+bp) | pareados (reais) | '
          'sem-par (reais) | % | destino dominante | sugestão |')
        w('|---|---|--:|--:|--:|---|---|')
        for m in sorted(by_crate[crate]):
            rec = mods[m]
            reais = rec['reais_total']
            bp = rec['bp_total']
            p = pct(rec['reais_pareados'], reais)
            dest, frac = dominant_dest(rec)
            dest_str = '—' if dest == '—' else f"{dest} ({int(round(frac*100))}%)"
            w(f"| `{m}` | {reais}+{bp} | {rec['reais_pareados']} | "
              f"{rec['reais_sempar']} | {int(round(p*100))}% | "
              f"{dest_str} | {classify_module(rec)} |")
        w('')

    # ---- Censo inverso: cristalino novo ----
    census = build_cristalino_census(itens)
    w('## Censo inverso — módulos do cristalino com itens sem-par '
      '(o que é novo)')
    w('')
    w('Apenas censo (sem coluna declarada): itens do cristalino sem '
      'correspondente no vanilla, por módulo.')
    w('')
    w('| módulo cristalino | itens novos (reais+bp) |')
    w('|---|---|')
    for m in sorted(census):
        c = census[m]
        w(f"| `{m}` | {c['reais']}+{c['bp']} |")
    w('')
    w(f"Total cristalino sem-par: "
      f"{sum(c['reais'] for c in census.values())} reais + "
      f"{sum(c['bp'] for c in census.values())} bp.")
    w('')

    return '\n'.join(out)


def main():
    src = sys.argv[1] if len(sys.argv) > 1 else '/tmp/comparar-typst-itens.json'
    with open(src) as f:
        data = json.load(f)
    sys.stdout.write(emit(data))
    sys.stdout.write('\n')


if __name__ == '__main__':
    main()
