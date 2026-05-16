# Relatório do passo P260 — ADRs meta formalizando padrões "auditoria condicional" + "diagnóstico imutável" cumulativos

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-260.md`.
**Tipo**: passo arquitectural meta administrativo XS;
**não materializa código**.
**Análogo estrutural canónico**: P156K (criou ADR-0064 +
ADR-0065 simultaneamente, ambas `EM VIGOR`, formalizando
padrões N≥5).
**Magnitude planeada**: XS.
**Magnitude real**: **XS (~15 min)** — passo administrativo
puramente documental.

---

## §1 — Sumário executivo

**ADRs criadas**: **2** (ADR-0084 + ADR-0085).
**Status**: ambas `EM VIGOR`.
**Tests delta**: **2334 verdes preservado** (zero alteração;
passo puramente documental).

**ADRs tocadas/criadas**:
- **ADR-0084** (nova) — "Auditoria condicional — audit empírico
  antes de decisão B1/B2/B3"; formaliza padrão **N=5** das
  aplicações P192A/P255/P257/P258/P259.
- **ADR-0085** (nova) — "Diagnóstico imutável — artefacto
  produzido por audit"; estende ADR-0034; formaliza padrão
  **N=4** dos diagnósticos imutáveis P255/P257/P258/P259.

**ADRs distribuição**:
- PROPOSTO 11 (inalterado).
- EM VIGOR 30 → **32** (+0084, +0085).
- IMPLEMENTADO 25 (inalterado).
- Total 70 → **72**.

**Ficheiros criados**:
- `00_nucleo/adr/typst-adr-0084-auditoria-condicional.md`.
- `00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md`.
- `00_nucleo/materialization/typst-passo-260-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/adr/README.md` (tabela + distribuição + entrada
  P260 nos passos-chave).

**Zero código L1/L2/L3/L4 tocado**.
**Zero prompts L0 editados**.
**Zero DEBTs criados/encerrados**.

---

## §2 — Sub-passo P260.A — Inventário inline

### A.1 — "Auditoria condicional" N=5 cumulativo

Aplicações empíricas confirmadas:

| Passo | Módulo/alvo | Cenário aplicado |
|-------|-------------|------------------|
| P192A | M7 fixpoint runtime | B1 retroactivo (estruturalmente fechado) |
| P255 | DEBT-8 Math (4 pendências) | B1 fecho (4/4 cumulativo) |
| P257 | Color paridade vanilla | Fase A literal (8/8 + 4 scope-outs) |
| P258 | Model (22 entradas) | B1 fecho conceptual (~73%) |
| P259 | Visualize (27 subsistemas) | B2 sub-passos adiados (~52%) |

Ficheiros relatório confirmados:
- `typst-passo-192a-relatorio.md`.
- `typst-passo-255-relatorio.md`.
- `typst-passo-257-relatorio.md`.
- `typst-passo-258-relatorio.md`.
- `typst-passo-259-relatorio.md`.

### A.2 — "Diagnóstico imutável precedente à acção" N=4 cumulativo

Ficheiros diagnósticos imutáveis confirmados:

| Passo | Ficheiro |
|-------|----------|
| P255 | `00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md` |
| P257 | `00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md` |
| P258 | `00_nucleo/diagnosticos/diagnostico-model-fase-a-passo-258.md` |
| P259 | `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md` |

Todos marcados explicitamente "Imutável após criação per
ADR-0034".

### A.3 — Slots ADR 0084 + 0085 livres

```bash
$ ls 00_nucleo/adr/typst-adr-*.md | grep -oE "0[0-9]{3}" | sort -u | tail -5
0079
0080
0081
0082
0083

$ ls 00_nucleo/adr/typst-adr-0084* 00_nucleo/adr/typst-adr-0085*
ls: não foi possível acessar [...]: Arquivo ou diretório inexistente
```

→ Confirmação: último ADR existente = 0083 (P257); slots 0084
e 0085 livres.

**Auto-aplicação ADR-0065 critério #5 cumprida** — inventário
inline antes da criação das ADRs.

---

## §3 — Sub-passo P260.B — ADR-0084 criada

Ficheiro novo:
`00_nucleo/adr/typst-adr-0084-auditoria-condicional.md`.

**Status**: `EM VIGOR`.

**Estrutura**:
- Contexto — N=5 cumulativo com tabela 5 aplicações
  (passo + módulo + estado declarado pré + estado factual
  pós + cenário).
- Decisão — pattern formalizado como metodologia canónica:
  estrutura obrigatória (.A audit + .B reconciliação + .C
  materialização condicional + .D fecho); critério "cobertura
  ambígua" (gatilho); decisão B1/B2/B3 (limiares como
  guidelines, não absolutos).
- Compatibilidades — ADR-0034 + ADR-0065 + ADR-0085 + política
  P158 "sem novas reservas".
- Consequências — positivas (reduz obsolescência; acelera
  decisões; cross-references estáveis; honestidade documental)
  + negativas (overhead ~30-45 min; risco audit superficial
  mitigado por checklist literal) + neutras.
- Alternativas — ADR única vs separação P156K-style;
  decisão adoptada: separação.
- Referências — ADR-0034/0065/0085/0064 + 5 relatórios
  aplicações.
- Auto-aplicação — P260 cumpre ADR-0065 critério #5 inline.

---

## §4 — Sub-passo P260.C — ADR-0085 criada

Ficheiro novo:
`00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md`.

**Status**: `EM VIGOR`.
**Estende**: ADR-0034 (diagnóstico canónico).

**Estrutura**:
- Contexto — N=4 cumulativo com tabela 4 ficheiros imutáveis
  (passo + ficheiro + função).
- Decisão — diagnósticos audit Fase A são artefactos imutáveis;
  propriedades obrigatórias (1-7: localização canónica + nome
  + marcador imutabilidade + cabeçalho + conteúdo literal +
  tabelas estruturadas + decisão B1/B2/B3); diferença vs
  diagnósticos transientes (tabela 4 tipos); distinção crítica
  vs ADR-0034 (âmbito: tipo vanilla vs audit empírico).
- Consequências — positivas (evidência preservada;
  cross-references estáveis; auditoria de processo) + negativas
  (sem actualização — audit futuro produz novo ficheiro) +
  neutras (coexistência com diagnósticos preparatórios).
- Alternativas — actualização permitida vs revisão -R1
  ADR-0034 vs nova ADR; decisão adoptada: ADR-0085 nova
  estendendo ADR-0034.
- Referências — ADR-0034/0029/0084 + 4 ficheiros aplicações.
- Auto-aplicação — P260 não produz diagnóstico Fase A
  (administrativo XS); inventário inline aceitável per
  ADR-0065 §"Neutras".

---

## §5 — Sub-passo P260.D — README ADRs actualizado

### Tabela ADRs

Duas entradas adicionadas após ADR-0082:
- ADR-0084 `EM VIGOR` P260.
- ADR-0085 `EM VIGOR` P260.

### Distribuição

- PROPOSTO 11 (inalterado).
- IDEIA 2 (inalterado).
- **EM VIGOR 30 → 32** (+0084, +0085 P260).
- IMPLEMENTADO 25 (inalterado).
- REVOGADO 2 (inalterado).
- ADIADO 1 (inalterado).
- **Total: 70 → 72**.

### Total nota actualizada

Linha "**Total**: 65 ADRs..." actualizada para incluir
"**+ADR-0084 + ADR-0085 EM VIGOR P260** ... **Total pós-P260:
72 ADRs**".

### Passos-chave — entrada P260 adicionada

Entrada nova ~40 linhas descritivas com sumário canónico:
- Tipo (administrativo XS).
- ADRs criadas (0084 + 0085).
- Auto-aplicação ADR-0065 critério #5 inline §A.
- Numeração escolhida (0084+0085 consecutivos).
- Contagens (70→72; EM VIGOR 30→32).
- Zero código; tests 2334; lint zero violations.
- Benefício (overhead reduzido; rastreabilidade formal).
- Subpadrão emergente "passo administrativo XS criar/promover
  ADR" N=3→N=4 cumulativo (ADR-0062-create + P156K + P160A +
  **P260**).
- 45 aplicações cumulativas anti-inflação preservadas.
- Marco P260; política P158 preservada.

---

## §6 — Padrões metodológicos

### Auto-aplicação P156K-style

P156K é o **template literal** deste passo. Paridade:

| Aspecto | P156K | P260 |
|---------|-------|------|
| Tipo | Administrativo XS | Administrativo XS |
| ADRs criadas | ADR-0064 + ADR-0065 | ADR-0084 + ADR-0085 |
| Ambos status | `EM VIGOR` | `EM VIGOR` |
| Padrão formalizado N | 6 (Smart→Option) + 5 (Inventariar) | 5 (Auditoria condicional) + 4 (Diagnóstico imutável) |
| Auto-aplicação | Sim | Sim (P260 cumpre ADR-0065 critério #5 inline) |
| Código alterado | Zero | Zero |
| Tests delta | Zero | Zero |
| Magnitude | XS | XS |

### Subpadrão "passo administrativo XS criar/promover ADR" N=3 → N=4

Cumulativo:
- N=1 ADR-0062-create (2026-04-27).
- N=2 P156K (criou 0064+0065 simultaneamente; 2026-04-26).
- N=3 P160A (criou ADR-0066 PROPOSTO; 2026-04-30).
- **N=4 P260** (cria ADR-0084+0085 simultaneamente; este
  passo; 2026-05-15).

**Patamar N=4 atinge limiar formalização consolidado** (N=3-4
política). Candidato a meta-meta-ADR futuro — **improvável e
desnecessário** pois padrão é auto-documentado em cada
aplicação.

### Subpadrões "Auditoria condicional" e "Diagnóstico imutável" PROMOVIDOS

Antes deste passo:
- "Auditoria condicional" N=5 (P192A + P255 + P257 + P258 +
  P259).
- "Diagnóstico imutável precedente à acção" N=4 (P255 +
  P257 + P258 + P259).

Após este passo:
- Ambos **promovidos a ADRs `EM VIGOR`** (ADR-0084 + ADR-0085).
- Audits futuros consumirão ADRs explicitamente em vez de
  re-justificar empiricamente; cada novo audit cresce N=5→6
  e N=4→5 sem renegociação metodológica.

---

## §7 — Cobertura

**Inalterada** — passo administrativo XS. Nenhum módulo recebe
expansão substantiva. Cobertura agregada cristalina por área
preservada:

- Math: DEBT-8 ENCERRADO P255 (~95%).
- Color: 100% estrutural P257.
- Model: ~73% pós-P258.
- Visualize: ~52% pós-P259.
- Layout Fase 5: ~98-99% P253 IMPLEMENTADO.
- Cobertura agregada user-facing total: ~75-76% preservado.

---

## §8 — Limitações e trabalho futuro

### Padrões formalizados consumíveis em audits futuros

Audits futuros candidatos (não-bloqueantes; recomendações
subjectivas):

1. **Text P262?** — Text/Markup módulo cobertura ambígua;
   aplicação directa ADR-0084 (Fase A audit) + ADR-0085
   (diagnóstico imutável).
2. **Layout final P262'?** — Layout cobertura ~98-99% mas
   audit final pode confirmar saturação formal (Cenário B1
   esperado).
3. **P-Footnote-N refino M** — não exige audit (refino
   pontual; M magnitude).
4. **P261 Paint enum + Gradient Linear** (sequência preferida
   pós-P259 Cenário B2) — refino visualize concreto.

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. ADRs criadas
formalizam padrões empíricos existentes; não criam reservas
futuras.

### Sem DEBT novo aberto

Saldo DEBTs preservado.

### Possível meta-meta-ADR futuro (improvável)

Subpadrão "passo administrativo XS criar/promover ADR" N=4
cumulativo (limiar atingido); candidato a formalizar mas
**desnecessário** pois padrão é auto-documentado em cada
aplicação (cada passo administrativo cita análogo estrutural
canónico — P156K — directamente).

---

## §9 — Critério de aceitação global P260 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found` (esperado "Nothing to fix" — passo
  administrativo).
- [x] `cargo test --workspace --release` retorna **2334 verdes
  preservado** (sem regressão).
- [x] `typst-adr-0084-auditoria-condicional.md` criado;
  status `EM VIGOR`.
- [x] `typst-adr-0085-diagnostico-imutavel.md` criado; status
  `EM VIGOR`.
- [x] `README.md` ADRs actualizado (distribuição 70 → 72;
  entrada P260).
- [x] Relatório criado em `00_nucleo/materialization/`.
- [x] Zero código L1/L2/L3/L4 tocado.
- [x] Zero prompts L0 editados.
- [x] Zero DEBTs criados/encerrados.
- [x] Política "sem novas reservas" preservada (formaliza
  padrões empíricos; não cria reservas).

**Estado pós-P260**:
- Tests workspace: **2334 verdes preservado**.
- Hash drift: zero.
- Lint: zero violations ("Nothing to fix").
- DEBTs saldo: **10 preservado**.
- ADRs distribuição: PROPOSTO 11; IDEIA 2; **EM VIGOR 32**;
  IMPLEMENTADO 25; REVOGADO 2; ADIADO 1; **total 72**.
- Prompts L0 editados: 0.
- Diagnóstico imutável criado: 0 (passo administrativo XS).
- ADRs criadas: 2 (`EM VIGOR` ambas).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P260**: 2 padrões meta consolidados em ADRs `EM VIGOR`
reduzem overhead documental cumulativo para audits futuros;
política "sem novas reservas" P158 preservada (P260 formaliza
padrões empíricos existentes, não cria reservas).

**Recomendação subjectiva pós-P260**:

- **P261 Paint enum + P262 Gradient Linear** (sequência
  preferida P259 Cenário B2; M+S+; ~+11pp Visualize cobertura).
- **OU Text audit** (P262? — aplicação directa ADR-0084 + 0085).
- **OU P-Footnote-N** refino M (P258 pendência residual).

**Decisão humana fica em aberto literal** pós-P260.

---

## §10 — Referências

- `CLAUDE.md` — Protocolo de Nucleação.
- **ADR-0084** (criada P260) — Auditoria condicional `EM VIGOR`.
- **ADR-0085** (criada P260) — Diagnóstico imutável `EM VIGOR`;
  estende ADR-0034.
- ADR-0034 — Diagnóstico canónico (estendido por ADR-0085).
- ADR-0064, ADR-0065 — Precedente canónico P156K (template
  formato auto-aplicação).
- ADR-0083 — Color paridade vanilla (último ADR pré-P260).
- Relatórios aplicações "Auditoria condicional":
  - P192A — `typst-passo-192a-relatorio.md`.
  - P255 — `typst-passo-255-relatorio.md`.
  - P257 — `typst-passo-257-relatorio.md`.
  - P258 — `typst-passo-258-relatorio.md`.
  - P259 — `typst-passo-259-relatorio.md`.
- Diagnósticos imutáveis aplicações:
  - P255 — `diagnostico-math-fase-a-passo-255.md`.
  - P257 — `diagnostico-color-vanilla-passo-257.md`.
  - P258 — `diagnostico-model-fase-a-passo-258.md`.
  - P259 — `diagnostico-visualize-fase-a-passo-259.md`.
- P156K — precedente "passo administrativo XS criar ADRs
  meta simultaneamente" (N=2 cumulativo).
- P160A — precedente "passo administrativo XS criar ADR
  PROPOSTO" (N=3 cumulativo).
- ADR-0062-create — N=1 precedente "passo administrativo XS
  criar ADR".
- **P260 — N=4 cumulativo** (este passo).
