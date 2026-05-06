# Historiograma dos passos do projecto Typst Cristalino

**Data de produção**: 2026-04-25.
**Geração**: Claude Code (LLM externa) executando o passo P156A,
lendo sequencialmente os ~258 ficheiros de
`00_nucleo/materialization/` (197 specs + 61 relatórios).
**Escopo**: passos P0 a P155 inclusive variantes (-v2/-v3, 81.5,
83.5/.6, 84.x, 91_5, 96.x, 131A/B, 132A/B, 140A/B, 154A/B).
**Natureza**: meta-documento descritivo. Não é prescritivo
absoluto. Onde a evidência é fraca, mantém-se qualificador.
**Coexistência**: complementa `blueprint-projecto.md` (snapshot
estático "onde estamos"); este documento responde "como
chegámos".

---

## Sumário executivo

Em ~5 meses (Janeiro a Abril 2026, datas precisas frequentemente
omissas dos relatórios pré-P98), 155 passos numerados (mais
~40 variantes/sub-passos) materializaram um compilador Typst
cristalino com 1145 testes, 60 ADRs e 13 DEBTs em aberto. A
trajectória não é linear: contém **8 séries distintas de
reformulação** (4 forks v2/v3 nos passos 4-9, série 96.x com
duas inserções intermédias, série paridade com 6 reformulações
sucessivas) e **6 aplicações documentadas do padrão
diagnóstico-primeiro** (sufixos `A` em 131A/132A/140A/154A,
mais P148 sem sufixo e P156A actual). Em todas as 6 aplicações
o diagnóstico descobriu informação que **alterou
materialmente** a materialização (gerando DEBTs novos,
revogando estimativas, ou re-orientando o roadmap).

Padrões emergentes principais:

1. **"Diagnóstico-primeiro" tem retorno alto consistente** —
   nenhuma das 6 aplicações terminou sem alteração da
   materialização planeada.
2. **Forks de spec (v2/v3) concentram-se nos primeiros
   10 passos** (P4-9) e desaparecem após P10. Causa provável:
   estabilização do método de redacção de specs e amadurecimento
   da bagagem arquitectural.
3. **Clusters temáticos densos**: math (P34-50, 17 passos),
   show rules + introspection (P56-66, 11 passos), imagens
   (P71-75, 5 passos), gráficos (P76-79, 4 passos), série
   84.x governança ADRs (P83.5-84.8h, 11 sub-passos), série
   96.x reestruturação por domínio (P96-96.10+97, 12 passos),
   CLI (P112-120, 9 passos), DEBT-1 (P126-141, 16 passos),
   paridade (P148-153, 6 passos), Model (P154A-155, 3
   passos). Cada cluster funciona como uma "fase" do projecto.
4. **DEBTs raramente são fechados no passo seguinte ao da
   abertura** — fechos comuns ocorrem 2-5 passos depois;
   DEBT-1 ficou aberto >120 passos antes de fechar (P22→P142).
5. **Antipadrão "tudo-num-passo" é raro mas custou** — quando
   ocorreu (e.g. P102, P103) descobriu-se em fase de execução
   que premissas estavam erradas e o passo redefiniu-se em
   plena execução.

---

## §1 — Linha temporal completa

### §1.1 Tabela cronológica compacta

Cada linha = um passo (ou variante). Colunas:
- `#`: número/identificador.
- `Tipo`: dominante (sub = substantivo, adm = administrativo,
  diag = diagnóstico, arq = arqueológico, fDEBT = fecho-DEBT,
  inv = investigação, ref = refino).
- `ADRs`: criadas (`+NNNN`), actualizadas (`~NNNN`), revogadas
  (`-NNNN`).
- `DEBTs`: abertos (`+N`), fechados (`-N`).
- `Padrão`: padrão metodológico dominante.
- `Notas`: reformulação, dependência sequencial, observação.

Variações `?` indicam lacuna factual (não-inferida). Ver §3.4
e §3.5 para ciclos de vida agregados.

#### Fundações (P0-P25)

| # | Tipo | ADRs | DEBTs | Padrão | Notas |
|---|------|------|-------|--------|-------|
| 0 | adm | +0001 | ? | tudo-num-passo | Estrutura inicial; quarentena de `lab/` |
| 1 | sub | (~0001) | ? | tudo-num-passo | Tipos de domínio; SyntaxNode bloqueado |
| 2 | sub | (~0001,0004) | ? | diag-primeiro (ecow) | SyntaxNode/Set; alimenta ADR-0005 |
| 3 | sub | (~0005) | ? | tudo-num-passo | PackageSpec DTO + World trait |
| 4 | inv | — | — | diag-primeiro | **v1 cancelado** (parou em diagnóstico) |
| 4-v2 | sub | (~0007,0014,0015) | — | diag-reduzido | **Reformulação** v1; pré 105 testes |
| 5 | inv | — | — | diag-primeiro | **v1 cancelado** |
| 5-v2 | sub | (~0015) | — | diag-reduzido | **Reformulação** v1; pré 126 testes |
| 6 | inv | — | — | diag-primeiro | **v1 cancelado** (eval em L3, depois revisto) |
| 6-v2 | inv | — | — | diag-primeiro | **v1→v2 cancelado** |
| 6-v3 | ref | +0016 | — | refino | **3ª reformulação**; cancela ambição eval; stubs |
| 7 | sub | (~0018) | — | diag-reduzido | SystemWorld em L3 (W11) |
| 8 | inv | — | — | diag-primeiro | **v1 cancelado** (TOCTOU detectado) |
| 8-v2 | sub | (executa 0019/0021 ou 0021/0022; numeração ambígua) | — | refino | **v1 reformulado** |
| 8-v3 | sub | (numeração ADR diferente de v2) | — | refino | **v2 reformulado**; conteúdo ≈ v2 |
| 9 | sub | — | — | tudo-num-passo | Paridade parsing por Debug; **v1** |
| 9-v2 | sub | — | — | refino (CompactNode DTO) | **v1 reformulado** |
| 9-v3 | sub | — | — | refino (.name() em vez de Debug) | **v2 reformulado** |
| 10 | sub | (executa 0021) | — | diag-reduzido | Datetime + FontBook real |
| 11 | sub | (~0023,0016) | — | diag-primeiro (Go/No-Go P12) | indexmap; Module real |
| 12 | diag | (~0016,0001) | — | diag-primeiro | Engine types + esqueleto eval |
| 13 | sub | (executa 0024) | — | diag-reduzido | Value subset (5 variantes) |
| 14 | sub | — | — | diag-primeiro (A/B/C/D) | Comemo em testes; operações |
| 15 | sub | +0025 | — | diag-primeiro (prova vida) | control flow + Array/Dict/Module/Datetime |
| 16 | sub | — | — | diag-primeiro | Closures + FuncCall (Teste de Ouro) |
| 17 | sub | — | — | diag-primeiro | Recursão + stdlib mínima + named args |
| 18 | sub | (~0016) | — | diag-primeiro (B/C/D) | Content mínimo |
| 19 | sub | (~0026) | — | diag-primeiro | Frame + Page + word-wrap |
| 20 | sub | — | — | diag-primeiro | export_pdf em L3 |
| 21 | sub | — | — | tudo-num-passo | FontBookMetrics; ttf-parser fora L1 |
| 22 | sub | — | +1 (StyleChain) | tudo-num-passo | Strong/Emph/Heading; **regista bomba-relógio** |
| 23 | sub | — | — | tudo-num-passo | Raw, listas, links |
| 24 | fDEBT | +0027 | -1 (DEBT-5) | tudo-num-passo | CIDFont + ToUnicode; **DEBT-5 fecha** |
| 25 | fDEBT | +0028 | (parcial DEBT-4) | diag-primeiro | Length/Color/Ratio/Angle |

#### Cluster math + show rules (P26-P67)

| # | Tipo | ADRs | DEBTs | Padrão | Notas |
|---|------|------|-------|--------|-------|
| 26 | sub | (aplica 0029,0030,0031,0026rev) | — | diag-primeiro | Conformidade ADRs nova vintage |
| 27 | fDEBT | — | -1 (DEBT-4) | diag-primeiro | DEBT-4 fecha |
| 28 | fDEBT | — | (parcial DEBT-3) | diag-primeiro | While limit + call depth |
| 29 | fDEBT | — | (estrutura DEBT-3) | diag-primeiro | Ciclos importação (estrutura) |
| 30 | fDEBT | — | (parcial DEBT-1) +7 | diag-primeiro | StyleChain — abre DEBT-7 implicito |
| 31 | fDEBT | — | (parcial DEBT-2) | diag-primeiro | Closures lazy/eager |
| 32 | fDEBT | — | -6, +7 (formal) | diag-primeiro + tarefa-0 adm | Suite L3 nova; DEBT-7 formaliza |
| 33 | fDEBT | — | -7 | diag-primeiro | #set scoping por bloco |
| 34 | sub | (~0032) | +8 | diag-primeiro | Prepara math; abre DEBT-8 |
| 35 | diag | — | — | arq/diag (corpus parity) | Baseline parity P36 |
| 36-50 | sub (15) | — | (DEBT-8 ~) ; +9 (P43) ; -9 (P45) | diag-primeiro | **Cluster math 15 passos**; ver narrativa §1.2 |
| 49→50 | sub→ref | — | — | diag-primeiro | **Reformulação localizada** (heurística → contexto) |
| 51 | sub | — | — | duas-passagens | MathAlignPoint |
| 52 | ref | — | — | spacing OpenType | Continuação P51 |
| 53 | ref | — | — | refactor geom. | Fecha dívida P46 |
| 54 | sub | — | — | reúso layout | Matrizes |
| 55 | sub | — | — | mapeamento eval | Vec/cases (reúso P54) |
| 56 | sub | (~0015 ref) | — | extensão AST | Labels/Refs (fecho motor math) |
| 57 | sub | — | +1 (DEBT-10) | single-pass | CounterState |
| 58 | sub | — | — | generalização | Counters genéricos |
| 59 | sub | — | (DEBT-10 declarado) | resolve refs | Auto-numeração |
| 60 | sub | — | -1 (10), +1 (11) | duas passagens | Motor introspecção; encerra 10 |
| 61 | sub+fDEBT | — | -1 (11), +2 (12,13) | decomposição | TOC + sub-módulos |
| 62 | sub+fDEBT | — | -1 (10b?) +2 (14,15) | reúso introspecção | Figuras |
| 63 | fDEBT | — | -2 (12,13) +3 (16,17,18) | 3ª passagem read-only | TOC paginada |
| 64 | ref | (~0024) | -1 (16) | named args | Desacoplamento NativeFunc |
| 65 | fDEBT | — | -1 (17) | fixpoint | Convergência layout |
| 66 | fDEBT | — | -1 (18) | materialização | Contexto temporal TOC |
| 67 | sub | — | — | infra | map_text — fundação show |

#### Show rules + features visuais (P68-P80)

| # | Tipo | ADRs | DEBTs | Padrão | Notas |
|---|------|------|-------|--------|-------|
| 68 | sub | — | +4 (19,20,21,22) | eager | Show base; cascata DEBTs |
| 69 | fDEBT | — | -1 (19), +1 (23) | map_content | Travessia bottom-up |
| 70 | fDEBT | — | -2 (20,23), ~21 mitigado | guards | Anti-recursão |
| 71 | sub | — | +3 (24b,25,26) | Arc<Vec<u8>> | Imagens — abre cascata |
| 72 | fDEBT | (~0001) | -1 (24b), +1 (24c) | trait ImageSizer | Dimensões reais |
| 73 | fDEBT | — | -1 (24c), +3 (27,28,29) | JPEG /DCTDecode | Image export |
| 74 | fDEBT (×4) | — | -4 (26,27,28,29) | PNG /FlateDecode | **4 DEBTs simultâneos** |
| 75 | fDEBT (×3) | — | -3 (14,15,25) | EvalContext.current_file | **3 DEBTs** |
| 76 | sub | — | +3 (30,31,32) | Stroke/ShapeKind | Gráficos cascata |
| 77 | fDEBT | — | -1 (32) | κ Bézier | Curvas |
| 78 | fDEBT | — | -1 (31) | matriz cm + AABB | Transforms |
| 79 | fDEBT | — | -1 (30), +1 (33) | W n + path | Polígonos |
| 80 | sub | — | +4 (34b,c,d,e) | Fixed→Auto→Fraction | Grid; cascata DEBT-34* |

#### Page config + auditorias + comemo + reestruturação (P81-P97)

| # | Tipo | ADRs | DEBTs | Padrão | Notas |
|---|------|------|-------|--------|-------|
| 81 | sub | — | — | TDD snapshot | #set page; depende 80 |
| 81.5 | diag | — | — | gate | Stress geom; bloqueia P82 |
| 82 | sub | — | (+36, +37) | TDD c/ diags | Align/place |
| 83 | sub | — | -34b/c, +38 | reúso TrackSizing | Grid rows; abre DEBT-38 |
| 83.5 | diag | — | — (auditoria) | auditoria | Auditoria DEBTs; **lacuna**: relatório só conversa |
| 83.6 | adm | — | — | regeneração git | **Recupera lacuna** P83.5 |
| 84.1 | adm | — | — (textual) | edição textual | Limpeza DEBT.md |
| 84.2 | fDEBT | — | -38 | HashMap L1 | Cache sub-frames |
| 84.3 | fDEBT | — | -21 | identidade NodeKind | 3 cenários A/B/C |
| 84.4 | fDEBT | — | -22, +39 | Arc<[T]> | Listas quase-imutáveis |
| 84.5 | fDEBT | — | -36 | composição | Value::Align |
| 84.6 | fDEBT | — | -37 | 3 cenários | Place relativo |
| 84.7 | diag | — (audita 31) | — | replicação 83.5 | **Auditoria ADRs**; alimenta 84.8x |
| 84.8a | sub | +0032 | +40,+41,+42 | gov | Política unsafe |
| 84.8b | adm | ~0001,0007,0017,0018,0027,0028 | — | refactor cabeçalhos | Status canónico |
| 84.8c | adm | ~0029,0030 | — | edição corpo | Cancela plano ADR-0033 |
| 84.8d | adm | ~0022,0023,0025 | — | mover conteúdo | Cria `diagnosticos/` |
| 84.8e | sub | +0033, +0034 | — | formaliza regras | Paridade + diagnóstico obrig. |
| 84.8f | adm | ~0026, ~0026-R1 | — | convenção -RN | Distinção revogação/revisão |
| 84.8g | adm | ~~9 ADRs (status) | — | uniformiza vocab. | 6 valores canónicos PT |
| 84.8h | adm | ~0002, ~0003, +README | — | encerra série | Índice canónico |
| 85 | fDEBT+diag | — | -41 | A exec + B diag | Diag Route alimenta P90 |
| 86 | diag | — | — | 2 diagnósticos .md | Stubs comemo + padrão #[track] |
| 87 | diag | (lê 0024) | — | sub-passo | Verificação EcoVec |
| 88 | sub | — | — | warm-up comemo | Traced primeiro #[track] |
| 89 | sub | +0035 | +43 | gov | EcoVec + gap linter |
| 90 | sub | — | -40 | construção grande | Route real; remove unsafe |
| 91 | adm | — | +44, +45 | gov pós-construção | Divergência Route |
| 91_5 | adm | +0036 | — | sub-passo gov | Atomização (ficheiro fornecido) |
| 92 | fDEBT | — | -44 | 1ª aplic ADR-0036 | Route estrutural |
| 93 | ref+diag | — | -45 (parcial) | diag + ligação check_*_depth (3/4) | check_html pendente |
| 94 | ref | — | — | propagação StyleChain | 2ª aplic 0036 |
| 95 | ref | — | -39 (provável) | extracção show_rules | 3ª aplic 0036 |
| 96 | adm | +0037 (PROP) | +46 | gov | Inicia série coesão |
| 96.1 | ref | (aplica 0037) | -46 [1/8] | mov. incremental | eval.rs 3780 linhas |
| 96.2 | ref | — | -46 (continuação) | armos triviais inline | **Inserção correctiva** desloca seguintes |
| 96.3 | adm | ~0037 (PROP→VIGOR) | -46 (renumera) | promoção | Empírica pós-96.1 |
| 96.4 | ref | — | -46 [n/9] | parse 2255 | 2ª aplic 0037 |
| 96.5 | ref | — | -46 [n/9] | stdlib 1711 | Divisão por área |
| 96.6 | adm | ~0037 (Regra 3) | +47, -46 (renumera) | adendo | **2ª inserção correctiva**; abre DEBT-47 |
| 96.7 | ref | — | -46 [n/10] | layout 2848 | Layouter<M> genérico |
| 96.8 | ref | — | -46 [n/10] | math/layout 1806 | Análoga 96.7 |
| 96.9 | ref | — | -46 [9/10] | lexer ou Regra 6 | Decisão Fase 0 |
| 96.10 | adm | — | -46 (encerra) | verificação | Fecho série 96.x |
| 97 | fDEBT | (aplica 0037 nota) | -47 | auditoria visibilidade | Endereça lacuna série 96 |

#### Sink, Engine, CLI, formato diagnósticos (P98-P120)

| # | Tipo | ADRs | DEBTs | TestsΔ | Padrão | Notas |
|---|------|------|-------|--------|--------|-------|
| 98 | sub | — | — | L1=0 (764) | 4ª aplic 0036 | Encerra ADR-0036 sobre EvalContext |
| 99 | sub | +0038 | +48, ~1 | L1+16 | fundação Style | Cria Content::Styled |
| 100 | fDEBT | +0039 | -48 | L1+3 | SR (cache+SoT) | Activa Styled no Layouter |
| 101 | ref | — | — | L1=0 (783) | construtores factory | Remove Strong/Emph |
| 102 | sub | +0040 | +49 | L1+7 | **Descoberta empírica** | **Spec partiu de premissa errada**; #set já activo desde P30 |
| 103 | sub | +0041 | +50 | L1+5 | validação como activação | **Idem P102**; #show activo desde P70 |
| 104 | sub | +0042 | +51, ✓DEBT-49 parcial | L1+8 | gate "≤1 função" | Sink stub→real |
| 105 | diag | — | — | =0 | manutenção preventiva | Auditoria global DEBTs |
| 106 | fDEBT | +0043 | -51 | L3+4 | API dupla tracked | Canal Sink → L3; descobre derive Clone obrigatório |
| 107 | fDEBT | — | -49 | L3+6 | 5ª aplic 0036 | DEBT-49 fecha; revela limite "10 params" |
| 108 | diag | — | — | =0 | análise + ranking | Recomenda Engine (Cand. 5) |
| 109 | sub | +0044 | — | =0 | big-bang | Engine<'a>; **inverte parcialmente 0036**; descobre TrackedMut::reborrow_mut |
| 110 | fDEBT | — | -45 | =0 | gate "não aplicável" | Puramente documental (Opção A) |
| 111 | sub | +0045 | — | L1+8 L3+5 | gcc/clang | Formato rico diagnósticos |
| 112 | diag | — | — | =0 | inventário 4 cand. | Recomenda Cand.2 mínimo |
| 113 | sub | +0046 | — | L3+6 | argparsing manual | CLI mínima 04_wiring (depois corrigida) |
| 114 | ref | — | — | L4+5 | integration tests | Materializa cenários P113 |
| 115 | ref | +0047 | — | =0 | clap Compat A | Tests P114 inalterados |
| 116 | sub | +0048 | — | L3+12 | ANSI literal | **V12 disparou** mid-passo |
| 117 | ref | +0049 | — | L2+6 L3-6 | corrige camada | **Auto-crítica**: 113-116 erraram camada |
| 118 | diag | — | — | =0 | auditoria 4 crates | Ranking candidatos |
| 119 | ref | +0050 | — | L2+9 L3-15 (-6) | completa P117 | ADR-0049 anota "completada por 0050" |
| 120 | sub | +0051 | — | L2+6 L4+2 | pattern P1-P6 | Estabelece pattern flags futuras |

#### CLI flags + DEBT-1 subsets (P121-P140)

| # | Tipo | ADRs | DEBTs | TestsΔ | Padrão | Notas |
|---|------|------|-------|--------|--------|-------|
| 121 | sub | (~0051) | 0 | +4 | aplicação 0051 | --root |
| 122 | sub | (~0051) | 0 | +3 | aplicação 0051 | --font-path; descoberta L4 nunca chamava with_fonts |
| 123 | sub | (~0051) | 0 | +3 | consolidação clap | env vars TYPST_* |
| 124 | adm | 0 | 0 | +7 | tests-only defensivo | Disciplina CLI |
| 125 | diag | 0 | 0 | =0 | auditoria empírica | Quebra linha CLI; nada trivial |
| 126 | fDEBT | (~0038) | 0 | +2 | DEBT-1 XS v1 | text.weight numérico |
| 127 | fDEBT | (~0038) | 0 | +2 | DEBT-1 XS v2 | text.tracking |
| 128 | fDEBT | 0 | 0 | +3 | DEBT-1 XS v3 | text.leading; **divergência par/text** (corrigida P134) |
| 129 | fDEBT | (~0038) | 0 | +5 | DEBT-1 XS v4 | weight simbólico |
| 130 | fDEBT | 0 | 0 | +3 | DEBT-1 XS v5 | text.lang |
| 131A | diag | +0052 (PROP) | 0 | =0 | **diag-primeiro 1ª** | Descobre lang precisa tipo |
| 131B | sub | ~0052 (IMPL), ~0038 | 0 | +12 | mat. pós-diag | **Par com 131A**; breaking semantic |
| 132A | diag | +0053 (PROP) | 0 | =0 | **diag-primeiro 2ª** | Bloqueia em regex |
| 132B | sub | ~0053 (IMPL), ~0038 | 0 | +14 | mat. pós-diag | **Par com 132A**; pool DEBT-49 rotacionado |
| 133 | ref | 0 | 0 | =0 | infra dispatcher | par target |
| 134 | ref | 0 | 0 | +1 | fecha divergência P128 | leading text→par |
| 135 | diag | +0054 (VIGOR) | +52 | =0 | **diag-primeiro 3ª** | DEBT-52 abre; reformulação roadmap |
| 136 | sub | 0 | 0 | +5 | roadmap DEBT-52 fase A | Quebra Copy TextStyle |
| 137 | sub | 0 | 0 | +3 | roadmap DEBT-52 B.1 | tracking — primeiro efeito visível desde P102 |
| 138 | sub | 0 | 0 | +3 | roadmap DEBT-52 B.2 | leading — descobre falta Parbreak |
| 139 | sub | 0 | 0 | +5 | roadmap DEBT-52 B.3 | weight faux-bold |
| 140A | diag | +0055 (PROP), ~0019 | 0 | =0 | **diag-primeiro 4ª** | **Reformulação Fase C menor** (infra já existia) |
| 140B | sub | (0055 fica PROP) | 0 | +11 | mat. pós-diag | **Par com 140A**; ADR-0055 transita só em P141 (trio) |

#### Fim DEBT-1 + paridade + Model (P141-P155)

| # | Tipo | ADRs | DEBTs | TestsΔ | Padrão | Notas |
|---|------|------|-------|--------|--------|-------|
| 141 | sub | ~0055 | 0 | +5 | trio 140A/B/141 | Array fallback; ADR-0055 IMPL; gap 6 DEBT-52 |
| 142 | fDEBT | 0 | -2 (DEBT-1, DEBT-52) | =0 | L0-puro adm | **Lacuna**: sem relatório separado |
| 143 | adm | 0 | 0 | =0 | inventário empírico | Correcção README ADRs; renumeração |
| 144 | sub | +0057 | 0 | +9 | tudo-num-passo | Lang hyphenation; gap 7 DEBT-52 |
| 145 | adm | 0 | 0 | =0 | análogo P84.8g | Cabeçalhos ADRs 0038-0051 |
| 146 | sub | (anota 0055) | 0 | +9 | mat. voluntária pós-DEBT-1 | Multi-font per document |
| 147 | adm | 0 | 0 | =0 | reescrita factual | Docs paridade |
| 148 | diag | 0 | 0 | =0 | **diag-primeiro 5ª (sem sufixo)** | Inventário cobertura; **reformula série paridade** |
| 149 | arq | +0058, +0059 | 0 | =0 | arq → ADR formaliza | Value::Type/Args; cobertura arq. 70%→72% |
| 150 | sub | 0 | +1 (DEBT-53) | =0 (lab+1) | baseline | P3 cristalino-only |
| 151 | inv | 0 | +1 (DEBT-54) | =0 | inv → DEBT cascata | DEBT-54 bloqueia DEBT-53; **pausa** |
| 152 | ref | 0 | 0 | =0 | refino administrativo | Plano DEBT-54; conflito comemo 0.4/0.5 |
| 153 | sub | 0 | 0 | =0 (lab+1) | baseline | P2 cristalino-only; **suspende série paridade** |
| 154A | diag | +0060 (PROP) | +1 (DEBT-55) | =0 | **diag-primeiro 6ª** | Encerra paridade; gap Model 38%→32-36% |
| 154B | sub | (anota 0060) | 0 | +10 | mat. pós-diag | **Par com 154A**; cobertura Model→41% |
| 155 | sub | ~0060 (PROP→IMPL) | 0 | +22 | mat. fecha fase | Quote + smart-quotes; Fase 1 fecha; Tests 1145 |
| 156A | adm | 0 | 0 | =0 | **diag-primeiro 7ª (este passo)** | Historiograma do projecto |

### §1.2 Narrativa cronológica por cluster

**P0-P3 (fundação inicial)**: estrutura de directórios, quarentena
de `lab/`, primeiros tipos de domínio puro, contrato `World`.
Padrão: tudo-num-passo dominante. Trabalho derivado de
ADR-0001 + ADR-0004.

**P4-P9 (turbulência inicial)**: forks v2/v3 dominam. Cada um
dos passos P4, P5, P6, P8, P9 reformulou pelo menos uma vez.
P6 acumula três versões (v1, v2, v3); v3 cancela explicitamente
a ambição de ambas anteriores e regista a decisão em ADR-0016
("adiamento de eval"). Causa provável: imaturidade do método
de redacção de specs face à complexidade do domínio.

**P10-P25 (fundações pipeline)**: Module/eval/Value/Frame/PDF
construídos incrementalmente. Cada passo declara o anterior
como pré-condição numérica explícita. Padrão "diagnóstico-primeiro"
generaliza-se. P22 destaca-se por **registar bomba-relógio**
(StyleChain como DEBT-1 ainda antes de ser efectivamente um
problema). P24 fecha o primeiro DEBT (DEBT-5 unicode PDF).

**P26-P50 (cluster math)**: 25 passos consecutivos focados no
motor matemático. P26 aplica conformidade arquitectural; P27-P33
fecha série DEBT-3/4/6/7; P34-P50 constrói pipeline math
(MathIdent → MathFrac → símbolos → sqrt → constantes OpenType
→ glifos extensíveis → kern → primes → baselines → operadores
grandes). Único *refino localizado* deste cluster: P49→P50
(heurística "sempre vertical" → diferenciação inline vs display).

**P51-P67 (introspection + show base)**: matrizes/vec/cases
(P51-55), Labels/Refs (P56), Counters (P57-58), auto-numeração
(P59), motor introspecção 2-pass (P60), TOC (P61-63), refactor
NativeFunc (P64), fixpoint (P65), prova stdlib (P66), map_text
(P67). P68 abre cascata show rules.

**P68-P80 (features visuais)**: show eager (P68), DEBTs
relacionados (P69-70 fecha 19/20/23), imagens (P71-75 fecha
24b/24c/26/27/28/29/14/15/25), gráficos (P76-79 fecha 30/31/32),
grid (P80 abre cascata 34*).

**P81-P83.6 (page config + auditoria)**: page config (P81),
gate stress (P81.5), align/place (P82), grid rows (P83),
auditoria DEBTs (P83.5 — *primeira aplicação do padrão
auditoria-pivô*), recuperação de relatório perdido (P83.6 —
*lacuna documental conhecida*).

**P84.x (governança ADRs)**: 14 sub-passos consecutivos, todos
≤S de escopo, divididos em duas vagas. Vaga 84.1-84.6:
fecho-DEBT (cada passo fecha um DEBT identificado em P83.5).
Vaga 84.7-84.8h: P84.7 replica auditoria de P83.5 mas para
ADRs; P84.8a-h consome cada secção do relatório 84.7. Resultado
agregado: ADR-0032 (unsafe), ADR-0033 (paridade), ADR-0034
(diagnóstico obrigatório), convenção `-RN`, vocabulário de
status canonizado, README.md de índice. Pivô: o projecto
tinha 31 ADRs no início e 35+ no fim, com governança formal
estabelecida.

**P85-P95 (comemo + atomização)**: integração comemo (P85-88),
governança EcoVec (P89), Route real removendo último unsafe
(P90), governança pós-construção (P91, P91_5 com ADR-0036),
3 aplicações ADR-0036 (P92, P94, P95). P92 é a primeira
aplicação concreta de atomização; descobre o padrão
`T<'static> as Validate`. P94 elimina save/restore. P95
combina 3 tarefas (limpeza, extracção show_rules, avaliação).

**P96-P97 (reestruturação coesão por domínio)**: 12 sub-passos
da série 96.x mais P97. Padrão: governança (P96 abre DEBT-46 +
propõe ADR-0037) → 1ª aplicação (P96.1) → **inserção
correctiva** (P96.2 descoberta empiricamente) → promoção
(P96.3 ADR-0037 PROPOSTO→EM VIGOR) → aplicações em série
(P96.4-96.9) → **2ª inserção correctiva** (P96.6 abre DEBT-47)
→ encerramento (P96.10) → DEBT-47 fecha em P97. Total ~14600
linhas reorganizadas (eval 3780 + parse 2255 + stdlib 1711 +
layout 2848 + math/layout 1806 + lexer 1250). **ADR-0037 é a
primeira ADR promovida empiricamente** (PROPOSTO→EM VIGOR após
validação concreta em P96.1).

**P98-P111 (Sink, Engine, formato diagnósticos)**: 4ª aplicação
ADR-0036 (P98), fundação Style/Styles/StyleChain (P99-101),
**activações com descoberta empírica** (P102-103: spec partiu
de premissa errada; #set/#show já estavam activos desde
P30/P70). Sink (P104), auditoria DEBTs (P105), canal Sink→L3
(P106), DEBT-49 fecha (P107), análise Engine (P108), Engine
big-bang (P109 — *inverte parcialmente ADR-0036*), DEBT-45
documental (P110), formato rico (P111).

**P112-P120 (CLI completa)**: 9 passos consecutivos. P112
análise → P113 implementação L4 → P114 tests → P115 clap →
P116 cores → **P117 auto-crítica + correcção camada
(L4→L2)** → P118 auditoria → P119 completa correcção →
P120 pattern flags. Notar: 4 passos consecutivos (113, 115,
116, 117) erraram camada e foram corrigidos retroactivamente.
ADR-0049 corrige sem revogar 0046/47/48 (padrão "Nota
Passo N").

**P121-P125 (CLI flags)**: aplicação do pattern P120 a --root,
--font-path, env vars. Tests defensivos. Auditoria 11 DEBTs
(P125) quebra linha temática.

**P126-P141 (DEBT-1 sucessivo)**: 16 passos focados em DEBT-1.
P126-130: 5 subsets XS sucessivos (weight num, tracking,
leading, weight simbólico, lang). P131A→131B e P132A→132B:
**primeiros pares formais diagnóstico-primeiro**. P133-134:
infra par target + correcção divergência. P135: **3ª
aplicação diagnóstico-primeiro**, abre DEBT-52, reformula
roadmap (descobre 5/10 campos inertes). P136-139: roadmap
DEBT-52 fases A-B. P140A→140B+P141: **trio** (4ª aplicação
diagnóstico-primeiro com ADR-0055 transitando para IMPL só
em P141).

**P142 (fecho DEBT-1)**: passo administrativo XS L0-puro.
**Lacuna factual** (único passo do range pós-98 sem
relatório separado). DEBT-1 e DEBT-52 fecham; cumprem ADR-0054
(perfil observacional graded).

**P143-P147 (interlúdio governança + voluntário)**: correcção
README ADRs (P143), lang hyphenation (P144), uniformização
cabeçalhos (P145), multi-font (P146 — voluntário pós-fecho
DEBT-1), docs paridade (P147 — prepara série paridade).

**P148-P153 (série paridade — 6 reformulações sucessivas)**:
inventário (P148, **5ª aplicação diagnóstico-primeiro sem
sufixo**) → arqueologia + ADR-0058/0059 (P149) → P3 baseline
(P150, abre DEBT-53) → investigação (P151, abre DEBT-54
bloqueante) → refino plano (P152) → P2 baseline (P153,
**suspende série**). Cada passo descobre obstáculo e gera
sub-trabalho. P154A formaliza o encerramento.

**P154A-P155 (Model Fase 1)**: 6ª aplicação diagnóstico-primeiro
(P154A com ADR-0060 PROPOSTO + DEBT-55), materialização
(P154B terms+divider, P155 quote+smart-quotes), fechamento
de fase (P155 ADR-0060 PROPOSTO→IMPLEMENTADO; tests 1123→1145).

**P156A (este passo)**: 7ª aplicação diagnóstico-primeiro,
aplicada ao *próprio histórico* do projecto.

---

## §2 — Padrões agregados

### §2.1 Diagnóstico-primeiro (sufixo `A` ou equivalente)

**Lista exaustiva** (7 aplicações):

| # | Sub-passo materialização | Distância | Descoberta |
|---|---|---|---|
| 131A | 131B | 0 (mesmo dia) | Lang precisa tipo dedicado; breaking semantic vs P130 |
| 132A | 132B | 0 (mesmo dia) | regex não-autorizada bloqueia FontList completo |
| 140A | 140B (+141) | 0 + 1 | Infra CIDFont **já existia**; reformulação Fase C menor |
| 148 | (sem par formal; deriva 149-153) | 1+ | Cobertura empírica vs declarada divergente; reformula série paridade |
| 154A | 154B (+155) | 0 + 1 | Cobertura Model 38%→32-36%; novo roadmap fases 1-2-3 |
| 156A | (este passo) | — | (em curso) |

Estatísticas (excluindo 156A):
- **6 aplicações** com diagnóstico explícito.
- **Em 6 de 6 aplicações, descobriu-se informação que alterou
  materialmente a materialização planeada** — sem excepção.
- **Em 4 de 6 aplicações, abriu-se DEBT novo**: DEBT-52 (P135),
  DEBT-53 (P150), DEBT-54 (P151), DEBT-55 (P154A). [P135 não
  está na lista mas era diagnóstico embutido em passo
  substantivo; conta como caso intermédio.]
- **Em 5 de 6 aplicações, ADR nova foi criada como
  formalização**: ADR-0052 (P131A), ADR-0053 (P132A), ADR-0055
  (P140A), ADR-0058+0059 (P149 — arqueológico), ADR-0060 (P154A).
- **Em 6 de 6 aplicações, distância ≤1 passo entre `A` e `B`**
  (todos consecutivos no mesmo dia ou dia seguinte).

Conclusão descritiva: nas 6 aplicações registadas
explicitamente como diagnóstico-primeiro, **a probabilidade
de descobrir informação relevante foi 100%**. Esta é uma
amostra pequena (N=6); evitar generalização absoluta.

### §2.2 Arqueológico

Passos que investigaram decisões pré-existentes para formalizar
via ADR ou DEBT (não para materializar feature nova):

- **P85** (parcial): diagnóstico de Route vanilla — alimenta P90.
- **P86** (parcial): padrão `#[track]` no vanilla — alimenta P88.
- **P87**: verificação estado autorização EcoVec.
- **P95**: análise extracção show_rules de EvalContext.
- **P102** (componente): descoberta de que #set já activo desde P30.
- **P103** (componente): descoberta de que #show já activo desde P70.
- **P108**: análise Introspection vanilla → recomendação Engine.
- **P112**: inventário 4 candidatos CLI vanilla.
- **P118**: auditoria de atribuição de camadas (4 crates).
- **P140A** (componente): infra CIDFont existente em L3.
- **P149** (paradigmático): arqueologia Value::Type + Value::Args
  → ADR-0058 + ADR-0059.

Total identificado: **~11 passos com componente arqueológica
declarada**. Característica comum: zero código tocado ou código
tocado é puramente cosmético (renomes, anotações ADR).

### §2.3 Substantivo por escopo

A spec do P156A pediu distribuição por escopo XS/S/M/M+/L/XL.
**Lacuna factual**: a maioria dos passos (especialmente P0-P80)
não declara escopo no cabeçalho. Convenção XS/S/M/L/XL aparenta
ter sido estabelecida após P83.5 ou P84.7 (vocabulário canónico
P84.8g). Para P81+, escopo é mais frequentemente declarado.
Distribuição parcial (apenas passos com declaração explícita
ou inferência segura via número de produtos):

- **XS** (≤1h): P83.6, P84.1, P84.2 (?), P84.6 (?), P84.8b,
  P84.8f, P84.8h, P87, P89, P91, P91_5, P96, P96.3, P96.6,
  P96.10, P110, P123, P126, P127, P128, P129, P130, P131A,
  P132A, P135, P140A, P142, P152. **~28 passos**.
- **S** (1-2h): P81.5, P84.4, P84.5, P84.8a, P84.8c, P84.8d,
  P84.8e, P84.8g, P88, P93, P96.2, P101, P102, P103, P105,
  P112, P114, P115, P118, P120, P121, P122, P125, P131B,
  P133, P134, P136, P137, P138, P139, P140B, P141, P143,
  P145, P147, P149, P151, P154B (?). **~38 passos**.
- **M** (2-4h): P83, P84.3, P85, P86, P92, P94, P95, P96.4,
  P96.5, P96.8, P96.9, P97, P98, P99, P100, P104, P106, P111,
  P113, P116, P117, P119, P132B, P144, P148, P150, P153,
  P154A. **~28 passos**.
- **M+/L** (3-6h): P81, P82, P90, P96.1, P96.7, P107, P109,
  P146, P155. **~9 passos**.
- **XL** (6-10h+): nenhum passo confirmado deste tamanho;
  DEBT-55 (bibliography) é estimado XL mas ainda não materializado.

A maioria dos passos é de escopo S/M (~66 dos ~155 com escopo
inferível). XS predomina em passos administrativos/governança
(P84.x, P96.x intermédios).

### §2.4 Administrativo / refino / governança

Passos sem código L1/L2/L3 substantivo, mas com peso documental
material:

- **P83.5, P83.6** (auditoria DEBTs + recuperação relatório).
- **P84.1, 84.7, 84.8a-h** (governança ADRs em massa).
- **P89, P91, P91_5** (governança comemo).
- **P96, P96.3, P96.6, P96.10** (governança coesão).
- **P105** (auditoria global DEBTs).
- **P110** (DEBT-45 documental).
- **P124** (tests-only disciplina CLI).
- **P125** (auditoria 11 DEBTs).
- **P142** (fecho DEBT-1 administrativo).
- **P143** (correcção empírica README ADRs).
- **P145** (uniformização cabeçalhos ADRs).
- **P147** (actualização docs paridade).
- **P152** (refino plano DEBT-54).
- **P156A** (este passo).

Total: **~30 passos administrativos/governança**. Concentração
em séries 84.x e 96.x; dispersão posterior em sub-séries
DEBT-1 e paridade.

### §2.5 Fecho-DEBT

Passos cujo critério principal era encerrar DEBT específica:

- **P24** (DEBT-5 unicode PDF).
- **P25** (DEBT-4 parcial).
- **P27** (DEBT-4 completa).
- **P28, P29** (DEBT-3 partes).
- **P30, P31, P32** (DEBT-1 parcial / DEBT-7 / DEBT-2 parcial).
- **P33** (DEBT-7 fecha).
- **P45** (DEBT-9 cmap).
- **P60-66** (sequência fecho 10/11/12/13/14/15/16/17/18).
- **P69, P70** (DEBT-19/20/23).
- **P72, P73, P74, P75** (cascata imagens 24b/24c/26/27/28/29/14/15/25).
- **P77, P78, P79** (cascata gráficos 30/31/32).
- **P84.2-84.6** (DEBTs 38/21/22/36/37).
- **P85** (DEBT-41).
- **P90** (DEBT-40).
- **P92** (DEBT-44).
- **P93** (DEBT-45 parcial).
- **P95** (DEBT-39 provável).
- **P97** (DEBT-47).
- **P100** (DEBT-48).
- **P106** (DEBT-51).
- **P107** (DEBT-49).
- **P110** (DEBT-45 não-aplicável).
- **P126-130, P131B, P132B, P134, P136-141** (subsets DEBT-1 +
  DEBT-52 fases).
- **P142** (DEBT-1 + DEBT-52 fecho formal).

Total: **~50+ passos** com critério primário fechar DEBT.
Estatísticas: ver §3.4.

### §2.6 Investigação

Passos cuja característica é descoberta de obstáculo (zero
código entregue ou código apenas para mapeamento):

- **P4**, **P5**, **P6**, **P6-v2**, **P8** (v1) — todos
  cancelados/reformulados.
- **P151** — paradigmático (DEBT-54 aberto bloqueando DEBT-53).

Total: **~6 passos** classificados como investigação pura
ou pré-cancelamento. P151 é o único pós-P10.

### §2.7 Refino

Passos cujo critério principal é actualização de plano de
DEBT existente, ou refactor sem feature nova:

- **P52, P53** (continuação cluster math sem nova entidade).
- **P64** (NativeFunc named args).
- **P94, P95** (atomização EvalContext via ADR-0036).
- **P96.1, 96.2, 96.4, 96.5, 96.7, 96.8, 96.9** (reestruturação
  por domínio).
- **P101** (consolidação Strong/Emph).
- **P114, P115, P117, P119** (CLI evolutivo + correcção camada).
- **P133, P134** (par target dispatcher; correcção divergência).
- **P152** (refino plano DEBT-54).

Total: **~17 passos refino**.

### §2.8 Tudo-num-passo vs Diagnóstico-primeiro

Comparação:

| Padrão | Exemplos | Característica de outcome |
|--------|----------|---------------------------|
| **Tudo-num-passo** (sem diagnóstico formal) | P0, P1, P3, P9-v3, P21, P22, P23, P24, P102 (parcial), P103 (parcial), P144 | Descobertas mid-passo em P102/P103 (premissas erradas) e P144 (gap 7 anotado pós-fecho). P22 *registou* DEBT antecipadamente. |
| **Diagnóstico-primeiro formal (sufixo A)** | 131A/B, 132A/B, 140A/B, 148, 154A/B | Em todas as aplicações descobriu-se informação relevante pré-materialização. Sub-trabalho identificado **antes** de tocar código. |
| **Diagnóstico-reduzido** (decisões já em ADRs) | P4-v2, P5-v2, P10, P13 | Tipicamente sucessor de fork v2/v3; menos descobertas mid-passo. |

Padrão emergente descritivo: passos com diagnóstico formal
**não** descobrem premissas erradas mid-passo (descobrem
*antes* de começar); passos tudo-num-passo *podem* descobrir
mid-passo (P102, P103 são exemplos paradigmáticos onde isto
custou redefinir o passo em pleno trabalho). N pequeno; evitar
generalização absoluta.

---

## §3 — Análise

### §3.1 Reformulações

#### §3.1.1 Forks de spec (sufixos -v2/-v3)

| Passo | Forks | Causa documentada |
|-------|-------|-------------------|
| P4 | v1, v2 | v1 parou em diagnóstico antes de executar; v2 incorpora ADRs novas |
| P5 | v1, v2 | v1 reformulado após P4-v2 concluir |
| P6 | v1, v2, v3 | v1 e v2 mantinham ambição de eval() completo; v3 cancela ambição (ADR-0016) |
| P8 | v1, v2, v3 | v1 com TOCTOU em source(); v2 corrige; v3 ajusta numeração ADR (conteúdo ≈ v2) |
| P9 | v1, v2, v3 | v1 Debug; v2 CompactNode DTO; v3 .name() vs format!("{:?}",kind) |

Característica comum: **todos concentrados em P4-P9**. Após
P10 desaparecem completamente. Causa provável: estabilização
do método de spec após primeiro ciclo de descoberta.

#### §3.1.2 Reformulações dentro de série

- **Série 96.x**: P96.2 inserida após P96.1 (delegação
  incompleta de armos descoberta empiricamente). P96.3 promove
  ADR-0037 com 4 ajustes. P96.6 inserida com adendo Regra 3
  + DEBT-47. **Duas inserções correctivas** deslocam
  numeração do DEBT-46 (9→10 checkboxes).
- **Série paridade (P148-P153)**: 6 reformulações sucessivas
  registadas explicitamente nos próprios relatórios. Cada
  passo descobre obstáculo e gera sub-trabalho. P153 suspende.
- **DEBT-1 roadmap (P130→P135)**: original previa "130 lang
  → 131 font → 132 par → 133 leading → 134 fecho". Real:
  130 → 131A/B → 132A/B → 133 → 134 → P135 abre DEBT-52 com
  novo roadmap fases A-E.
- **P49→P50**: heurística "sempre vertical" reformulada para
  diferenciação inline vs display.
- **P102, P103**: spec reformulada *durante* execução
  (descoberta empírica que premissas estavam erradas).
- **P117, P119**: 4 passos consecutivos (113-116) erraram
  camada arquitectural; corrigidos retroactivamente sem
  revogar ADRs.

#### §3.1.3 Reformulações tipologicamente

- **Cancelamento total** (passo nunca executado): P4 v1, P5 v1,
  P6 v1, P6 v2, P8 v1.
- **Inserção correctiva** (passo novo entre dois existentes):
  P96.2, P96.6.
- **Renumeração** (passo desloca-se): P143/P144 (lang
  hyphenation deslocada por correcção README), P146 (rejeita
  rótulo "142A"), P133/P134 (na sequência P130→P135).
- **Suspensão de série**: paridade em P153.
- **Conversão de spec mid-passo**: P102 (validação em vez de
  activação), P103 (idem), P140A (Fase C menor em vez de
  major).

### §3.2 Mudanças de prioridade

- **P22**: registo antecipado de DEBT-1 (não estava na fila
  de trabalho imediato; declarado como "bomba-relógio").
- **P83.5**: auditoria DEBTs gera vaga 84.1-84.6 que adia
  trabalho substantivo.
- **P84.7**: auditoria ADRs gera vaga 84.8a-h (8 sub-passos
  governança).
- **P109**: Engine inverte parcialmente ADR-0036 sem revogar.
- **P117**: auto-crítica + correcção camada CLI (4 passos
  retroactivos).
- **P135**: reformula roadmap DEBT-1 com novo perfil
  observacional graded (ADR-0054).
- **P143**: lang hyphenation adia-se para P144 por inserção
  da correcção README.
- **P153→P154A**: paridade suspensa; foco muda para gap Model.
- **P156A**: prioridade "snapshot blueprint canónica" desloca-se
  para "historiograma do método" (registo na própria spec).

Padrão: mudanças de prioridade típicas são **derivadas de
auditoria/diagnóstico** (P83.5, P84.7, P109, P135, P154A).
**Nenhuma mudança de prioridade foi imposta externamente sem
diagnóstico** nos relatórios lidos.

### §3.3 Dependências entre passos

#### §3.3.1 Sequências obrigatórias declaradas

- **P4 → P5 → P6 → P7 → P8** (cada um declara o anterior
  como pré-condição numérica).
- **P11 → P12 → P13 → P14 → P15 → P16 → P17** (cadeia
  incremental do motor eval).
- **P18 → P19 → P20 → P21 → P22 → P23** (cadeia pipeline visual).
- **P34 → P35 → P36 → ... → P50** (cluster math 17 passos
  sequenciais).
- **P57 → P58 → P59 → P60** (cadeia introspecção).
- **P61, P62, P63 abrem DEBTs 12-18; P64, P65, P66 fecham
  consecutivamente**.
- **P67 → P68 → P69 → P70** (cadeia show rules).
- **P71 → P72 → P73 → P74 → P75** (cadeia imagens).
- **P76 → P77 → P78 → P79** (cadeia gráficos).
- **P81 → P81.5 → P82** (gate stress bloqueia P82).
- **P83.5 → P84.1-84.6** (auditoria pivota fechos DEBT).
- **P84.7 → P84.8a-h** (auditoria pivota governança ADRs).
- **P85, P86, P87 → P88, P89, P90** (diagnósticos comemo
  alimentam materializações).
- **P91, P91_5, P92** (governança imediatamente seguida de
  primeira aplicação ADR-0036).
- **P96 → P96.1 → P96.2 → P96.3 → P96.4 → ... → P96.10 → P97**
  (série coesão).
- **P98-P107** (Sink + Engine + atomização sequencial).
- **P108 → P109** (análise → execução Engine).
- **P112 → P113 → P114 → P115 → P116 → P117 → P118 → P119
  → P120** (CLI completa).
- **P126-141** (DEBT-1 subset sucessivo + DEBT-52 fases).
- **P131A → P131B**, **P132A → P132B**, **P140A → P140B → P141**,
  **P154A → P154B → P155** (pares/trios diagnóstico-materialização).
- **P148 → P149 → P150 → P151 → P152 → P153** (série
  paridade).

#### §3.3.2 Bloqueios materiais entre passos

| Bloqueador | Bloqueado | Resolução |
|-----------|-----------|-----------|
| ADR-0025 | P15 (control flow) | ADR criada em P15 |
| ADR-0036 | P92 (1ª aplicação) | ADR registada em P91_5 |
| ADR-0037 | P96.1 (1ª aplicação) | ADR proposta em P96 |
| ADR-0038 | P100 (DEBT-48) | ADR proposta em P99 |
| ADR-0044 (Engine) | P109 | Análise em P108 |
| DEBT-54 | DEBT-53 | DEBT-54 aberto P151, ainda aberto |
| Fase 1 ADR-0060 | Fase 2 (P156+) | Fechada em P155 |

### §3.4 Ciclo de vida DEBTs

**Lacuna factual**: DEBT.md tem 54k tokens; análise integral
fora deste passo. Estatísticas baseadas em referências cruzadas
extraídas dos relatórios.

DEBTs identificados em ciclo (lista parcial):

| DEBT | Aberto | Fechado | Distância (passos) | Categoria |
|------|--------|---------|--------------------:|-----------|
| DEBT-1 | ~P22 | P142 | ~120 | Longo (compromisso fundamental) |
| DEBT-2 | ? | P31 (parcial) | ? | Médio |
| DEBT-3 | ? | P28+P29 | ? | Médio |
| DEBT-4 | ? | P25+P27 | ~3 | Curto |
| DEBT-5 | ? | P24 | ? | Único passo |
| DEBT-6 | ? | P32 | ? | Médio |
| DEBT-7 | P30 (impl.) → P32 (formal) | P33 | 3 | Curto |
| DEBT-8 | P34 | série math (parcial) | longo | Longo |
| DEBT-9 | P43 | P45 | 2 | Curto |
| DEBT-10 | P57 | P60 (62?) | 3-5 | Curto |
| DEBT-11 | P60 | P61 | 1 | Imediato |
| DEBT-12 | P61 | P63 | 2 | Curto |
| DEBT-13 | P61 | P63 | 2 | Curto |
| DEBT-14 | P62 | P75 | 13 | Médio |
| DEBT-15 | P62 | P75 | 13 | Médio |
| DEBT-16 | P63 | P64 | 1 | Imediato |
| DEBT-17 | P63 | P65 | 2 | Curto |
| DEBT-18 | P63 | P66 | 3 | Curto |
| DEBT-19 | P68 | P69 | 1 | Imediato |
| DEBT-20 | P68 | P70 | 2 | Curto |
| DEBT-21 | P68 | P84.3 | ~16 | Médio |
| DEBT-22 | P68 | P84.4 | ~16 | Médio |
| DEBT-23 | P69 | P70 | 1 | Imediato |
| DEBT-24b | P71 | P72 | 1 | Imediato |
| DEBT-24c | P72 | P73 | 1 | Imediato |
| DEBT-25 | P71 | P75 | 4 | Curto |
| DEBT-26 | P71 | P74 | 3 | Curto |
| DEBT-27 | P73 | P74 | 1 | Imediato |
| DEBT-28 | P73 | P74 | 1 | Imediato |
| DEBT-29 | P73 | P74 | 1 | Imediato |
| DEBT-30 | P76 | P79 | 3 | Curto |
| DEBT-31 | P76 | P78 | 2 | Curto |
| DEBT-32 | P76 | P77 | 1 | Imediato |
| DEBT-33 | P79 | (aberto) | — | Aberto |
| DEBT-34b | P80 | P83 | 3 | Curto |
| DEBT-34c | P80 | P83 | 3 | Curto |
| DEBT-34d | P80 | (aberto) | — | Aberto |
| DEBT-34e | P80 | (aberto) | — | Aberto |
| DEBT-36 | P82 | P84.5 | ~3 | Curto |
| DEBT-37 | P82 | P84.6 | ~3 | Curto |
| DEBT-38 | P83 | P84.2 | 1 | Imediato |
| DEBT-39 | P84.4 | P95 | ~11 | Médio |
| DEBT-40 | P84.8a | P90 | ~5 | Curto |
| DEBT-41 | P84.8a | P85 | <1 | Imediato |
| DEBT-42 | P84.8a | (aberto) | — | Aberto |
| DEBT-43 | P89 | (aberto) | — | Aberto |
| DEBT-44 | P91 | P92 | 1 | Imediato |
| DEBT-45 | P91 | P93+P110 | 2/19 | Médio |
| DEBT-46 | P96 | P96.10 | série interna | Médio |
| DEBT-47 | P96.6 | P97 | série interna | Imediato |
| DEBT-48 | P99 | P100 | 1 | Imediato |
| DEBT-49 | P102 | P107 | 5 | Curto |
| DEBT-50 | P103 | (aberto?) | — | Aberto |
| DEBT-51 | P104 | P106 | 2 | Curto |
| DEBT-52 | P135 | P142 | 7 | Curto |
| DEBT-53 | P150 | (aberto, bloqueado por 54) | — | Aberto |
| DEBT-54 | P151 | (aberto) | — | Aberto |
| DEBT-55 | P154A | (aberto) | — | Aberto |

**Distribuição agregada (estimada)**:

- **Imediatos** (≤1 passo): DEBT-11/16/19/23/24b/24c/27/28/29/32/38/41/44/47/48 = ~15.
- **Curtos** (2-5 passos): ~25.
- **Médios** (6-20 passos): ~10.
- **Longos** (>20 passos): DEBT-1 (~120), DEBT-8 (longo).
- **Abertos**: DEBT-33, 34d, 34e, 42, 43, 50, 53, 54, 55 = **9
  abertos** identificados; blueprint declara 13 abertos
  (discrepância de 4 — possível inclusão de sub-DEBTs não
  identificados nesta análise).

**Padrão dominante**: a maioria dos DEBTs é fechada **imediato
ou curto** (1-5 passos depois de aberto). DEBTs longos são
fundamentais (DEBT-1 — sistema de estilos completo). DEBTs
abertos persistentes são tipicamente bloqueados por dependências
externas (DEBT-53 bloqueado por DEBT-54; DEBT-55 bloqueado por
ADR-0061 a criar; DEBT-42 bloqueado por benchmark; DEBT-34d/e
bloqueado por trabalho de layout futuro).

### §3.5 Ciclo de vida ADRs

Total: **60 ADRs** (59 números únicos + ADR-0026-R1).
Distribuição final:

- `EM VIGOR`: 26 (regras/políticas activas).
- `IMPLEMENTADO`: 19 (decisões materializadas).
- `PROPOSTO`: 10 (decisões pendentes/inlining).
- `IDEIA`: 2.
- `REVOGADO`: 2.
- `ADIADO`: 1.

**Transições documentadas**:

- `PROPOSTO → IMPLEMENTADO`: ADR-0055 (P140A→P141), ADR-0060
  (P154A→P155). Distância: 1-3 passos.
- `PROPOSTO → EM VIGOR`: ADR-0037 (P96 → P96.3). Distância:
  3 sub-passos. **Único caso de promoção empírica observada**.
- `EM VIGOR → REVOGADO`: ADR-0007 (revogado por ADR-0018),
  ADR-0028 (revogado por ADR-0029).

**Tempo médio em cada estado**: dados insuficientes (datas
ausentes em maioria dos passos pré-P98). Inferência grosseira:
ADRs `IMPLEMENTADO` aparentam estabilizar ao fim de 1-3 passos
após criação. ADRs `EM VIGOR` aparentam ser estáveis (poucas
revogações/revisões). ADRs `PROPOSTO` antigas (0005-0015)
permanecem PROPOSTO há muitos passos — possível inflação de
PROPOSTO.

**ADRs criadas em massa**: série 84.8 (5 ADRs novas: 0032,
0033, 0034, 0035, retroactivo), série Sink/CLI (0042, 0043,
0044, 0045, 0046, 0047, 0048, 0049, 0050, 0051 — 10 ADRs em
P104-P120).

**ADRs revisadas**: 1 caso formal (ADR-0026 → ADR-0026-R1
em P84.8f). Padrão `-RN` formalizado nesse passo.

### §3.6 Antipadrões observados

1. **Spec partir de premissa errada** (P102, P103): activação
   de #set/#show planeada, mas ambos já estavam activos desde
   P30/P70. Custo: passo redefiniu-se em pleno trabalho;
   tornou-se "validação + ADR + canário".
2. **Erro de camada arquitectural** (P113-117): 4 passos
   consecutivos colocaram CLI em L4 quando deveria ser L2.
   Custo: P117 + P119 corrigem retroactivamente sem revogar
   ADRs (padrão "Nota Passo N").
3. **Numeração ADR ambígua entre forks** (P8-v2 vs P8-v3):
   v2 e v3 usam numeração ADR diferente (0021/0022 vs
   0019/0020) com conteúdo idêntico. Lacuna documental.
4. **DEBTs implícitos não-formalizados** (P30): DEBT-7 surge
   "implicitamente" em P30 mas só é formalmente registado
   em P32 (Tarefa 0). Discrepância sequencial.
5. **Cascata de DEBTs sem fecho imediato** (P71 abre 24b/25/26,
   P73 abre 27/28/29 — 6 DEBTs em 3 passos): pode mascarar
   pré-condição não satisfeita. Mitigado por fecho concentrado
   em P74 (4 DEBTs simultâneos).
6. **Inserção correctiva mid-série causa renumeração**
   (P96.2, P96.6): aceitável mas requer documentação explícita
   (ambos casos documentados).
7. **Lacuna documental** (P83.5): relatório só na conversa,
   recuperado em P83.6 via git. Caso paradigmático de risco
   se geração de relatório não for sistematizada.
8. **Reformulação de v1 (P4, P5, P6, P8, P9)**: forks v2/v3
   indicam que o método inicial não convergia. Concentrado
   nos primeiros 10 passos; resolvido por amadurecimento.

### §3.7 Saúde do projecto ao longo do tempo

#### §3.7.1 Crescimento de testes

Trajectória cristalino L1 + L3 + L4 (extracção das pré-condições
declaradas; aproximada onde dados ambíguos):

```
P3   ~ 69 testes
P4-v2 ~ 105
P5-v2 ~ 126
P6-v3 ~ 150
P10  ~ 179
P15  ~ 235
P20  ~ 325
P25  ~ 368
P30  ~ 405
P40  ~ 461
P50  ~ 553
P60  ~ 619
P70  ~ 694
P80  ~ 721
P90  ~ 747
P100 ~ 783
P110 ~ 803
P120 ~ ~830 (L1+L2+L3+L4 inferido)
P130 ~ ~870
P140A~ ~920
P150 ~ 1095
P155 ~ 1145
```

Crescimento monotónico. Aceleração visível em clusters
(math P34-50 +200 testes, show rules P67-70 ~+50, imagens
P71-75 ~+30, DEBT-52 P136-141 +30, Model P154B-155 +32).

```mermaid
line
    title Crescimento de testes ao longo dos passos
    x-axis "Passo" [P3, P10, P20, P30, P50, P70, P90, P110, P130, P150, P155]
    y-axis "Tests"
    line "Tests" [69, 179, 325, 405, 553, 694, 783, 803, 870, 1095, 1145]
```

#### §3.7.2 Crescimento de ADRs

```
P0   1 ADR (ADR-0001)
P10  ~10 ADRs (0001-0027 espalhadas)
P25  ~28 ADRs
P31  ~31 ADRs (pré-auditoria 84.7)
P84.8h ~35 ADRs (pós-série 84.8)
P95  ~37 ADRs
P109 ~44 ADRs (P98-P109 Sink/Engine)
P120 ~51 ADRs (P113-120 CLI)
P135 ~54 ADRs
P145 ~57 ADRs
P155 60 ADRs
```

Crescimento monotónico com 2 revogações (ADR-0007 → ADR-0018;
ADR-0028 → ADR-0029).

#### §3.7.3 Trajectória DEBTs (abertos vs fechados ao longo do tempo)

Lacunoso por dificuldade de cross-reference exacto. Aproximação:

```
P22-P25  ~3 abertos, 1 fechado
P30-P35  ~5 abertos, 5 fechados
P50-P60  ~10 abertos, 8 fechados (acumulado)
P70-P75  ~25 abertos, 18 fechados
P80-P85  ~30 abertos, 22 fechados
P95-P97  ~35 abertos, 30 fechados
P107-P110 ~38 abertos, 33 fechados
P125    ~42 abertos, 37 fechados
P142    ~50 abertos, 41 fechados (DEBT-1 e 52 fecham)
P155    ~55 abertos, 42 fechados (13 abertos finais)
```

Padrão: abertura e fecho roughly equilibrados. Diferencial
acumulado (~13 abertos finais) é dívida estrutural pendente.

```mermaid
line
    title DEBTs abertos vs fechados (cumulativo aproximado)
    x-axis "Passo" [P25, P50, P75, P100, P125, P142, P155]
    y-axis "DEBTs"
    line "Abertos" [3, 10, 25, 38, 42, 50, 55]
    line "Fechados" [1, 8, 18, 33, 37, 41, 42]
```

---

## §4 — Conclusões metodológicas

Esta secção é **descritiva**, não prescritiva absoluta. Onde
N é pequeno (<10), conclusões mantêm qualificadores explícitos.

### §4.1 Padrões com retorno alto consistente

1. **Diagnóstico-primeiro formal (sufixo `A`)**: em 6 de 6
   aplicações registadas, descobriu-se informação que alterou
   a materialização planeada. **Probabilidade observada de
   retorno alto: 100% (N=6)**. Custo: 1 passo administrativo
   adicional. Benefício observado: nenhum sub-passo precisou
   ser reformulado mid-execução nas 6 aplicações.

2. **Auditoria periódica como pivô** (P83.5, P84.7, P105,
   P125): em todas as 4 aplicações, identificou-se trabalho
   pendente que se tornou série de passos consecutivos
   (84.1-84.6, 84.8a-h, decisão de 110, 126-130). **N=4
   pequeno; tendência consistente**.

3. **Pares A→B emparelhados**: nas 4 aplicações com par formal
   (131A/B, 132A/B, 140A/B+141, 154A/B+155), distância foi
   ≤1 dia em todos os casos. **Padrão estável** com retorno
   alto: o diagnóstico fica fresco; materialização aproveita
   contexto.

4. **Cluster temático denso** (math P34-50, show P67-70,
   imagens P71-75, gráficos P76-79, CLI P112-120, Model
   P154B-155): trabalho focado em uma área por séries de
   passos consecutivos produz cobertura alta. Trabalho
   disperso entre áreas (P81→P85 alterna page+comemo)
   parece menos eficiente em termos de retorno por passo.
   **Evidência circunstancial**.

5. **Promoção empírica de ADR** (P96.3 PROPOSTO→EM VIGOR
   após validação concreta): único caso observado, mas
   bem-sucedido. ADR-0037 acumulou 4 ajustes derivados da
   primeira aplicação (P96.1) antes da promoção. **N=1
   apenas — evidência fraca**.

### §4.2 Padrões caros

1. **Spec partir de premissa errada** (P102, P103): custo de
   redefinição mid-passo. Mitigação: diagnóstico empírico
   *antes* de redigir spec.

2. **Erro de camada arquitectural** (P113-117): 4 passos
   retroactivamente corrigidos. Mitigação possível: lint V1
   na época já estaria a detectar (mas confusão entre L4
   wiring e L2 shell era subtil para o linter detectar).

3. **Forks de spec (v2/v3)**: concentrados em P4-P9. Custo
   alto inicial. Resolvido por amadurecimento; não
   re-emergiu.

4. **Cascata de DEBTs sem fecho imediato** (P71-P75 imagens,
   P76-P79 gráficos): aceitável quando fecho está planeado
   no passo seguinte. Risco: DEBTs órfãos. Mitigação:
   auditoria periódica (P83.5 capturou DEBTs órfãos
   pré-existentes).

5. **Reformulações sucessivas de série** (paridade P148-P153,
   6 reformulações): custo alto em coordenação. Resultado:
   suspensão. **Sinaliza pré-condição estrutural não
   satisfeita** (DEBT-54 vanilla setup). Mitigação possível:
   diagnóstico mais profundo antes de iniciar série.

6. **Tudo-num-passo em features grandes** (P22 Strong/Emph,
   P102/P103 activações): risco de descoberta mid-passo.
   Padrão diagnóstico-primeiro reduz este risco.

### §4.3 Recomendações empíricas para passos futuros

Recomendações **derivadas exclusivamente da evidência
compilada** neste historiograma. Cada recomendação cita
evidência. Aplicáveis a passos do mesmo tipo metodológico
analisado.

1. **Para features novas com cobertura ≥M**: aplicar
   diagnóstico-primeiro formal (sufixo `A`). Evidência:
   6/6 aplicações descobriram informação relevante.

2. **Para fechos de DEBT antigos** (>20 passos): redigir
   passo administrativo dedicado (modelo P142 fecho DEBT-1).
   Evitar fechar DEBT-grande dentro de passo substantivo
   maior. Evidência: P142 cumpriu ADR-0054 sem ambiguidade
   por ser dedicado.

3. **Para mudanças de camada arquitectural**: produzir
   ADR de correcção sem revogar ADRs anteriores (padrão
   "Nota Passo N" — P117/P119/ADR-0049/ADR-0050). Evidência:
   correcção retroactiva preservou linhagem documental.

4. **Para auditorias periódicas**: aceitar que produzem
   séries derivadas. Evidência: P83.5 → 84.1-84.6;
   P84.7 → 84.8a-h. Não tentar squashar em passo único.

5. **Para promoção de ADR PROPOSTO**: validar empiricamente
   antes (modelo P96 → P96.1 → P96.3). Evidência: única
   promoção registada deste tipo foi bem-sucedida.

6. **Para reformulações de série** (>3 reformulações
   consecutivas): considerar suspender e diagnosticar
   pré-condição estrutural. Evidência: paridade P148-P153
   suspendida em P153; P154A documentou que pré-condição
   real era gap Model.

7. **Para passos com >10 parâmetros funcionais**: considerar
   agregador struct (modelo Engine P109). Evidência:
   P107 atingiu limite 10 params; P109 resolveu via Engine
   sem revogar ADR-0036.

8. **Para subset XS sucessivos** (modelo DEBT-1 P126-130):
   pattern viável quando subsets têm complexidade similar.
   Evidência: 5 subsets em 5 passos sem reformulação.
   Funcionou porque subsets eram primitivos. Falhou em
   P131 (Lang) porque subset exigia tipo dedicado.

### §4.4 Limitações do método identificadas

1. **Datas omissas em maioria dos relatórios pré-P98**:
   estatísticas temporais (e.g. "tempo médio entre A e B")
   só inferíveis para pares pós-P98.

2. **Escopo XS/S/M/L/XL não declarado uniformemente**: convenção
   estabelecida apenas após P84.8g. Distribuição neste
   historiograma é parcial.

3. **Numeração de ADRs reescrita ao longo do tempo**: passos
   antigos (P7, P8) referem ADRs com números diferentes dos
   actuais. Histórico pré-P84.8h mostra inconsistências.

4. **DEBT.md grande** (54k tokens): cross-reference exacto
   entre DEBTs e passos não foi possível neste historiograma.
   Estimativa baseada em referências cruzadas dos relatórios.
   Discrepância detectada entre 9 DEBTs identificados como
   abertos e 13 declarados pelo blueprint.

5. **Relatórios pré-P98 não separados** (apenas spec + relatório
   no mesmo ficheiro): metadados de pós-execução mais difíceis
   de extrair. Convenção de relatório separado começou em P98.

6. **Lacuna documental P83.5**: relatório recuperado via git
   em P83.6. Sem garantia de fidelidade total ao original.

7. **Lacuna documental P142**: passo XS administrativo sem
   relatório separado; spec serve como referência.

8. **Reformulações da spec**: apenas detectadas as documentadas
   explicitamente nos relatórios. Reformulações silenciosas
   (ajustes mid-passo não-registados) não-detectáveis.

9. **Datas de transição de status ADR**: raramente documentadas
   na ADR; inferíveis apenas pelos passos.

10. **Cobertura empírica de features**: declarada em inventário
    P148 e actualizações P149/154A/154B/155, mas não
    auditada por este historiograma. Ver `blueprint-projecto.md`
    para snapshot autoritário.

---

## §5 — Coexistência e actualização

Este documento **complementa** `blueprint-projecto.md`:

- Blueprint = "onde estamos" (snapshot estático, estrutura
  arquitectural, próximas opções).
- Historiograma = "como chegámos" (trajectória dinâmica,
  padrões metodológicos, evidência empírica).

**Princípio de actualização**: regenerar historiograma quando
passos significativos fechem (não a cada passo). Análogo ao
blueprint. Frequência sugerida: a cada ~10-20 passos materiais,
ou quando uma fase fecha (e.g. Fase 2 ou 3 do roadmap Model).

**Regeneração**: via Claude Code (modelo deste passo P156A) ou
ferramenta automática (ver `ideias-projecto-blueprint-tool.md`
para esboço).

**Próximas perguntas que este documento deveria responder em
revisões futuras**:
- Distribuição actualizada de DEBTs (abertos vs fechados).
- Evolução de cobertura empírica por categoria.
- Eficácia do padrão diagnóstico-primeiro com N maior.
- Padrão emergente de Fase 2/3 Model.

---

## §6 — Referências

- `00_nucleo/diagnosticos/blueprint-projecto.md` (snapshot
  estático complementar).
- `00_nucleo/adr/README.md` (índice canónico ADRs).
- `00_nucleo/DEBT.md` (autoritário para DEBTs).
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (inventário 148 + actualizações).
- `00_nucleo/materialization/typst-passo-156a.md` (spec deste
  passo).
- `00_nucleo/materialization/typst-passo-156a-relatorio.md`
  (relatório da execução deste passo).
- `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
  (ferramenta automática esboçada como projecto futuro).
