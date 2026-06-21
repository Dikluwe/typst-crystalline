#!/usr/bin/env python3
"""
decompor_so_vanilla.py — Passo 385 (diagnóstico)

Decompõe o conjunto "só-vanilla" (itens.sem_par_antes da lente tekt-cargo-dsm)
nos baldes do Passo 385. A MEDIÇÃO É ESTE SCRIPT — determinística, re-rodável,
sem LLM no loop. O LLM entra só na proposta do marcador e no julgamento do
resíduo (balde 3), sempre como proposta a confirmar, nunca como autoridade da
contagem.

Entrada:  00_nucleo/diagnosticos/entrada-lente-so-vanilla.<data>.txt
          (TSV: kind \\t trait \\t nome \\t path — gerado de
           `lente --comparar --antes lab/typst-original --depois .`,
           campo itens.sem_par_antes; ver cabeçalho do .txt para proveniência)

Fontes de classificação (todas no repo, logo reprodutível):
  - L0  : 00_nucleo/prompts/**  (seções "## Sobre paridade")        -> balde 1
  - INV : 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md (148) -> balde 1
  - ADR : 00_nucleo/adr/*scope*/graded                              -> balde 2 (feature)
  - TOPO: tabela de crates fora-de-escopo (âncora escrita por crate) -> balde 2 (topologia)

Baldes (reconciliação fechada — somam o total):
  1  renomeado-com-registro     — símbolo migrado sob outro nome, âncora na literatura
  2  fora-de-escopo-com-ADR     — backends/tooling/vtable (topologia) + atributos graded (feature)
  M  mecânica-não-língua        — item Rust (trait-impl, método convencional, macro) cuja
                                  paridade NÃO se mede a este nível (ADR-0107); não é dívida
  3  resíduo-genuíno            — símbolo de nível-de-língua sem âncora e sem scope-out:
                                  o único insumo legítimo de migração

ADR-0107: paridade é com a LÍNGUA (semântica/sintaxe/morfologia), nunca com a
mecânica/igualdade do Rust. A lente pareia por chave K4 mecânica; por isso conta
como "só-vanilla" todo fn/método/trait-impl não-pareado mesmo quando a feature de
língua está migrada. O balde M isola essa mecânica — sem ele, o resíduo infla.

Ordem de classificação (primeiro match vence; ordem documentada):
  (1) crate em TOPO              -> balde 2-topologia
  (2) match na literatura L0/INV -> balde 1
  (3) match em token scope-out   -> balde 2-feature
  (4) regra de mecânica          -> balde M
  (5) senão                      -> balde 3

Uso:
  python3 decompor_so_vanilla.py [--entrada PATH] [--repo PATH] [--lista-residuo]
"""

import argparse
import os
import re
import sys
from collections import Counter, defaultdict

# ---------------------------------------------------------------------------
# TOPO — crates vanilla fora-de-escopo por construção, com âncora escrita.
# O cristalino tem exatamente 3 crates de produto (typst_core L1, typst_infra L3,
# typst_shell L2) + wiring. Estes crates vanilla não têm contraparte cristalina
# por decisão arquitetural; a âncora de cada um está no comentário.
# NÃO é estimativa herdada (§6): a contagem sai da varredura real da entrada.
# ---------------------------------------------------------------------------
SCOPE_OUT_CRATES = {
    "typst_pdf":    "ADR-0033 §res. visual (PDF diverge) / ADR-0075 (PDF backend externo)",
    "typst_svg":    "ADR-0033 §res. visual (SVG diverge) / ADR-0075 (SVG renderer externo)",
    "typst_html":   "ADR-0075 (HTML backend externo)",
    "typst_render": "ADR-0033 §res. visual (PNG/raster diverge) — backend de render",
    "typst_ide":    "topologia: cristalino não tem camada IDE (CLAUDE.md L1-L4)",
    "typst_docs":   "topologia: tooling de docs, não-compilador",
    "typst_timing": "topologia: instrumentação/timing, não-compilador",
    "typst_kit":    "topologia: I/O kit (download/fonts CLI); L3 reimplementa só o necessário",
    "typst_bundle": "topologia: bundling de assets/fonts, não-compilador",
    "typst_macros": "ADR-0026 + CLAUDE.md: sem proc-macros vtable",
    "typst_fuzz":   "topologia: harness de fuzz, não é produção",
    "test_wrapper": "topologia: harness de teste, não é produção",
}

# ---------------------------------------------------------------------------
# Métodos Rust convencionais (mecânica, não-língua). Quando o nome-folha de um
# fn é um destes, é dispatch/trait-glue do Rust, não uma função da linguagem
# Typst. Lista conservadora e auditável (ADR-0107).
# ---------------------------------------------------------------------------
RUST_METHOD_NAMES = {
    "new", "default", "fmt", "clone", "clone_from", "hash", "eq", "ne", "cmp",
    "partial_cmp", "from", "into", "try_from", "try_into", "as_ref", "as_mut",
    "as_str", "as_slice", "as_bytes", "deref", "deref_mut", "drop", "borrow",
    "borrow_mut", "to_owned", "to_string", "next", "next_back", "size_hint",
    "len", "is_empty", "iter", "iter_mut", "into_iter", "index", "index_mut",
    "add", "sub", "mul", "div", "neg", "not", "bitand", "bitor", "bitxor",
    "add_assign", "sub_assign", "mul_assign", "div_assign", "serialize",
    "deserialize", "visit_seq", "visit_map", "visit_str", "type_id",
    "provide", "source", "description", "cause",
}

# Palavras de path/literatura genéricas demais para serem chave de match.
STOPWORDS = {
    "self", "mod", "rs", "src", "crates", "lib", "typst", "library", "core",
    "layout", "model", "math", "text", "visualize", "foundations", "introspection",
    "Content", "Element", "Auto", "Smart", "None", "Some", "Vec", "Option", "Box",
}


def read_entrada(path):
    itens = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) != 4:
                # Falha visível (§8): não engole linha malformada.
                print(f"AVISO: linha ignorada (campos!=4): {line!r}", file=sys.stderr)
                continue
            kind, trait_, nome, p = parts
            itens.append({"kind": kind, "trait": trait_, "nome": nome, "path": p})
    return itens


def extrair_literatura(repo):
    """Tokens vanilla nomeados nas seções '## Sobre paridade' (L0) + colunas
    Vanilla do Inventário 148. Retorna (symbols:set, module_paths:set)."""
    symbols = set()
    module_paths = set()
    tok_re = re.compile(r"`([A-Za-z_][A-Za-z0-9_:/.]*)`")

    prompts_dir = os.path.join(repo, "00_nucleo", "prompts")
    for root, _dirs, files in os.walk(prompts_dir):
        for fn in files:
            if not fn.endswith(".md"):
                continue
            fp = os.path.join(root, fn)
            with open(fp, encoding="utf-8") as f:
                text = f.read()
            # extrai blocos "## Sobre paridade ..." até o próximo "## "
            for m in re.finditer(r"^##\s+Sobre\s+a?\s*paridade.*?$(.*?)(?=^##\s|\Z)",
                                  text, re.MULTILINE | re.DOTALL | re.IGNORECASE):
                bloco = m.group(1)
                for t in tok_re.findall(bloco):
                    _registrar_token(t, symbols, module_paths)

    inv = os.path.join(repo, "00_nucleo", "diagnosticos",
                       "typst-cobertura-vanilla-vs-cristalino.md")
    if os.path.exists(inv):
        with open(inv, encoding="utf-8") as f:
            for line in f:
                # tokens em backtick + tokens *.rs nus na coluna Vanilla
                for t in tok_re.findall(line):
                    _registrar_token(t, symbols, module_paths)
                for t in re.findall(r"\b([a-z_]+/[a-z_]+\.rs)\b", line):
                    _registrar_token(t, symbols, module_paths)
                for t in re.findall(r"\b([A-Z][A-Za-z0-9]*Elem)\b", line):
                    symbols.add(t)
    return symbols, module_paths


def _registrar_token(t, symbols, module_paths):
    if "/" in t or t.endswith(".rs"):
        # path de ficheiro vanilla, ex.: math/accent.rs  ->  modulo math::accent
        norm = t.replace(".rs", "").strip("/").replace("/", "::")
        if norm and norm not in STOPWORDS:
            module_paths.add(norm)
        return
    leaf = t.split("::")[-1]
    if leaf and leaf not in STOPWORDS and (leaf[0].isupper() or leaf.endswith("Elem")):
        # só símbolos (CamelCase/Elem); descarta minúsculas genéricas
        if len(leaf) > 2:
            symbols.add(leaf)


def extrair_scope_out_features(repo):
    """Tokens de atributos/feature declarados scope-out (graded) nas ADRs e
    nas linhas 'scope-out' do Inventário 148."""
    tokens = set()
    tok_re = re.compile(r"`([a-z_]{3,})`")
    adr_dir = os.path.join(repo, "00_nucleo", "adr")
    alvos = []
    for fn in os.listdir(adr_dir):
        if re.search(r"0054|0082|0083|0097", fn):
            alvos.append(os.path.join(adr_dir, fn))
    for fp in alvos:
        with open(fp, encoding="utf-8") as f:
            for line in f:
                low = line.lower()
                if "scope-out" in low or "scope out" in low or "graded" in low:
                    for t in tok_re.findall(line):
                        if t not in STOPWORDS:
                            tokens.add(t)
    # linhas scope-out do inventário
    inv = os.path.join(repo, "00_nucleo", "diagnosticos",
                       "typst-cobertura-vanilla-vs-cristalino.md")
    if os.path.exists(inv):
        with open(inv, encoding="utf-8") as f:
            for line in f:
                if "scope-out" in line.lower():
                    for t in tok_re.findall(line):
                        if t not in STOPWORDS:
                            tokens.add(t)
    return tokens


def crate_de(item):
    return (item["path"] or "").split("::")[0]


def modulo_de(item):
    # path = crate::a::b::Symbol  -> "a::b" (descarta crate e folha)
    segs = (item["path"] or "").split("::")
    if len(segs) <= 2:
        return ""
    return "::".join(segs[1:-1])


TYPE_LIKE = ("struct", "enum", "type", "trait", "const", "static")

# Módulos vanilla que materializam o sistema de elementos por VTABLE/proc-macro
# (`#[elem]`, `#[cast]`, Packed, ContentVtable, Reflect/IntoValue/FromValue).
# Scope-out declarado: ADR-0026 (Content enum fechado, sem vtable) + CLAUDE.md
# ("sem proc-macros vtable"); literatura locatable.md/element_kind.md/tag.md
# classifica marker-traits e vtable elements como scope-out cristalino.
VTABLE_MOD = (
    "typst_library::foundations::cast",
    "typst_library::foundations::content::element",
    "typst_library::foundations::content::field",
    "typst_library::foundations::content::vtable",
    "typst_library::foundations::content::packed",
    "typst_library::foundations::content::raw",
    "typst_library::foundations::content::capability",
)


def classificar(item, lit_symbols, lit_modules, scope_tokens):
    nome = item["nome"]
    leaf = nome.split("::")[-1]
    owner = nome.split("::")[0] if "::" in nome else ""
    kind = item["kind"]
    mod = modulo_de(item)

    # (1) crate fora-de-escopo (topologia/backend/vtable)
    crate = crate_de(item)
    if crate in SCOPE_OUT_CRATES:
        return ("2", "topologia", crate)

    # (2) mecânica do Rust independente de nome: implementação de trait e macro.
    #     ADR-0107: a paridade não se mede a este nível.
    if item["trait"]:
        return ("M", "trait-impl", item["trait"])
    if kind == "macro":
        return ("M", "macro", "")

    # (2b) sistema de elementos por vtable/proc-macro — scope-out ADR-0026.
    path = item["path"] or ""
    if any(path.startswith(m) for m in VTABLE_MOD):
        return ("2", "vtable-ADR-0026", "")

    # (3) renomeado-com-registro: o SÍMBOLO em si (tipo) está nomeado na
    #     literatura "Sobre paridade"/Inventário 148 sob (outro) nome.
    if kind in TYPE_LIKE and leaf in lit_symbols:
        return ("1", "literatura-simbolo", leaf)

    # (4) método de um tipo registrado na literatura = mecânica desse tipo,
    #     não um símbolo de língua próprio (ADR-0107).
    if kind == "fn" and owner and owner in lit_symbols:
        return ("M", "metodo-de-tipo-registrado", owner)
    if kind == "fn" and leaf in RUST_METHOD_NAMES:
        return ("M", "metodo-rust", leaf)

    # (5) módulo vanilla inteiro registrado como migrado (granularidade coarse).
    if kind in TYPE_LIKE and mod and mod in lit_modules:
        return ("1", "literatura-modulo", mod)

    # (6) atributo/feature scope-out (graded)
    snake = leaf if re.fullmatch(r"[a-z_]+", leaf) else ""
    if snake and snake in scope_tokens:
        return ("2", "feature", snake)

    # (7) resíduo genuíno
    return ("3", "residuo", "")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", default=os.path.abspath(
        os.path.join(os.path.dirname(__file__), "..", "..", "..")))
    ap.add_argument("--entrada", default=None)
    ap.add_argument("--lista-residuo", action="store_true",
                    help="imprime a lista completa do balde 3")
    args = ap.parse_args()

    repo = os.path.abspath(args.repo)
    entrada = args.entrada or os.path.join(
        repo, "00_nucleo", "diagnosticos",
        "entrada-lente-so-vanilla.2026-06-21.txt")

    itens = read_entrada(entrada)
    lit_symbols, lit_modules = extrair_literatura(repo)
    scope_tokens = extrair_scope_out_features(repo)

    baldes = defaultdict(list)
    sub = Counter()
    topo_por_crate = Counter()
    for it in itens:
        b, motivo, chave = classificar(it, lit_symbols, lit_modules, scope_tokens)
        baldes[b].append(it)
        sub[(b, motivo)] += 1
        if b == "2" and motivo == "topologia":
            topo_por_crate[chave] += 1

    total = len(itens)
    print(f"# Decomposição só-vanilla — total entrada = {total}")
    print(f"# literatura: {len(lit_symbols)} símbolos, {len(lit_modules)} módulos; "
          f"scope-out tokens: {len(scope_tokens)}")
    print()
    ordem = [("1", "renomeado-com-registro"),
             ("2", "fora-de-escopo-com-ADR"),
             ("M", "mecânica-não-língua (ADR-0107)"),
             ("3", "resíduo-genuíno")]
    soma = 0
    for b, nome in ordem:
        n = len(baldes[b])
        soma += n
        pct = 100.0 * n / total if total else 0
        print(f"balde {b}  {nome:42s} {n:6d}  ({pct:5.1f}%)")
    print(f"{'':9s}{'SOMA':42s} {soma:6d}")
    assert soma == total, f"RECONCILIAÇÃO FALHOU: {soma} != {total}"
    print("  reconciliação fechada ✓" if soma == total else "  ERRO")

    print("\n## sub-motivos")
    for (b, motivo), n in sorted(sub.items(), key=lambda kv: (-kv[1])):
        print(f"  balde {b:2s} {motivo:18s} {n:6d}")

    print("\n## balde 2 topologia — por crate (medido, não herdado)")
    for crate, n in topo_por_crate.most_common():
        print(f"  {crate:16s} {n:5d}   [{SCOPE_OUT_CRATES[crate]}]")

    # balde 3 por kind (separa língua de mecânica residual)
    print("\n## balde 3 (resíduo) por kind")
    for kind, n in Counter(it["kind"] for it in baldes["3"]).most_common():
        print(f"  {kind:8s} {n:6d}")

    # resíduo de nível-de-língua = struct/enum/type/trait (não-fn)
    lingua = [it for it in baldes["3"] if it["kind"] in ("struct", "enum", "type", "trait")]
    print(f"\n## resíduo de nível-de-língua (struct/enum/type/trait): {len(lingua)}")
    print("## resíduo de nível-fn (método/free-fn não-coberto pela regra): "
          f"{len(baldes['3']) - len(lingua)}")

    # Julgamento 4.3 (proposta determinística, não autoridade): separa
    # mecânica-de-execução (módulos de algoritmo cujo cristalino diverge de
    # propósito — ADR-0107) do candidato genuíno de língua. Marca inferência.
    # Mecânica de execução: módulos de algoritmo cujo cristalino diverge de
    # propósito (ADR-0107). Inclui o IR de math do vanilla (math::ir) e a
    # maquinaria de introspecção (builders/cache/*Introspection) — o cristalino
    # usa single-pass/enum-fechado, sem estes intermediários.
    EXEC_MOD = ("typst_layout::", "typst_eval::vm", "typst_eval::call",
                "typst_eval::flow", "typst_eval::access", "typst_eval::math",
                "typst_realize::", "typst_library::math::ir",
                "typst_library::introspection::introspector",
                "typst_library::introspection::convergence",
                "typst_library::introspection::query_",
                "typst_library::introspection::locator",
                "typst_library::introspection::location")

    def is_exec(it):
        p = it["path"] or ""
        return any(p.startswith(m) for m in EXEC_MOD)

    exec_l = [it for it in lingua if is_exec(it)]
    cand_l = [it for it in lingua if not is_exec(it)]
    print(f"\n## julgamento 4.3 do resíduo nível-de-língua (proposta, marca inferência)")
    print(f"  (c) mecânica-de-execução (layout/eval-vm/realize; ADR-0107): {len(exec_l)}")
    print(f"  (a/b) candidato genuíno de língua (a confirmar por humano):  {len(cand_l)}")
    print("  por crate::módulo-topo (candidato genuíno):")
    grp = Counter("::".join((it["path"] or "").split("::")[:2]) for it in cand_l)
    for k, n in grp.most_common(20):
        print(f"    {k:40s} {n:4d}")

    if args.lista_residuo:
        print("\n## LISTA balde 3 — candidato genuíno de língua (ordenada)")
        for it in sorted(cand_l, key=lambda x: x["path"]):
            print(f"  {it['kind']:7s} {it['path']}")
        print("\n## LISTA balde 3 — mecânica-de-execução (ADR-0107, não-dívida)")
        for it in sorted(exec_l, key=lambda x: x["path"]):
            print(f"  {it['kind']:7s} {it['path']}")


if __name__ == "__main__":
    main()
