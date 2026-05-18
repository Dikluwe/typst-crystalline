# ⚖️ ADR-0097: Scope-out reconfirmado por Fase A (NO-GO output legítimo)

**Status**: `EM VIGOR` (criada directamente; paridade pattern P271
ADR-0093/0094 — meta-ADR formalizando sub-padrão empírico N≥3
cumulativo)
**Data**: 2026-05-18
**Autor**: Humano + IA
**Validado**: Passo P273.17 — formaliza sub-padrão empírico N=3
cumulativo crossing limiar formalização N=3-4 com 3 razões NO-GO
distintas e legítimas.
**Diagnóstico prévio**: `00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`
§A.2.3 — verificação literal N=3 aplicações P273.14 + P273.15 + P273.16.
**Passo origem**: P273.17 (passo administrativo S+).
**Cluster**: Metodologia / Decisões / Fase A
**Tipo**: meta-ADR formalizando sub-padrão decisional empírico N=3 cumulativo

---

## Contexto

Pendências classificadas como **refinos qualitativos opcionais**
(per ADR-0054 graded) chegam à materialização com decisão Fase A
que pode legitimamente resultar em **NO-GO** quando análise empírica
revela bloqueador (externo, interno, ou estrutural).

**Spec workflow padrão** (precedente P273.14/15/16): passo entra com
"Fase A com verificação viabilidade go/no-go". Fase A produz
inventário factual + análise de risco + critério §A.4/§A.5; decisão
binária GO ou NO-GO. NO-GO **não é falha** — é **cumprimento honesto**
do critério de verificação empírica.

**3 aplicações cumulativas** (P273.14 → P273.15 → P273.16) consolidam
sub-padrão com **3 razões NO-GO distintas e legítimas**:

1. **P273.14 — CMYK-ICC paridade** — Razão NO-GO: **constraints
   externas**. Profile licensing (Adobe SWOP / ECI FOGRA / IDEAlliance
   GRACoL proprietários; zero royalty-free industry-recognized) +
   invariante L0 `export.md` linha 18 ("sem `crates` externas de
   PDF") + ICC.org Tech Note 7 explícito sobre ausência de profile
   CMYK genérico royalty-free.

2. **P273.15 — Bbox medido pós-layout** — Razão NO-GO: **constraints
   internas**. Zero demanda empírica em 8 sub-passos consecutivos
   (P273.6-P273.13 sem caso onde 3γ.2.γ produziu output incorrecto)
   + custo perf Caminho 1 eager O(N²) onde antes era O(N) + custo
   impl Caminho 2 lazy walker desproporcional sem demanda.

3. **P273.16 — Bbox.y topo-exacto inline** — Razão NO-GO:
   **bloqueador estrutural aceito**. P156H limitação consciente em
   L0 `content.md:817-829` ("`inset.top`/`inset.bottom` armazenados
   mas não aplicados em layout inline; alterariam line_height") +
   ADR-0078 §"Decisão" sub-fase (b) Opção A multi-region scope-out.
   **Descoberta empírica**: spec premissa "DEBT-56 EM ABERTO"
   factualmente desactualizada (DEBT-56 ENCERRADO P221, 2026-05-12).
   Fase A factual prevalece sobre premissa documental.

**N=3 cumulativo** com 3 razões distintas — limiar formalização
N=3-4 atingido.

### Marco metodológico

Sub-padrão empírico N=3 demonstra que **Fase A pode legitimamente
decidir NO-GO** com output documental em vez de materialização.
Pattern consolidado: Fase A factual + decisão binária + NO-GO output
documental (`trabalho-previo-externo.md`) + zero alterações código.

---

## Decisão

Para pendências classificadas como **refinos qualitativos opcionais**
(per ADR-0054 graded), **Fase A pode legitimamente decidir NO-GO**
com output documental em vez de materialização código.

### Quando aplicar NO-GO

A Fase A decide **NO-GO** quando empíricamente confirmado pelo menos
uma de **3 razões legítimas distintas**:

#### Razão 1 — Constraints externas

- Profile licensing, redistribuição, EULAs.
- Invariantes L0 do projecto (e.g. "sem crates externas de PDF").
- Crates não-autorizadas em [l1_allowed_external] ADR-0018.
- Decisão arquitectural maior fora do escopo do passo (e.g. ADR
  nova requerida como pré-requisito).

**Exemplo canónico**: P273.14 CMYK-ICC profile licensing + crate
externa + invariante L0.

#### Razão 2 — Constraints internas

- Custo performance inaceitável (pior caso quantificado, e.g. O(N²)).
- Ausência de demanda empírica concreta (zero casos registados em N
  sub-passos consecutivos).
- Custo de implementação desproporcional ao benefício sem demanda.
- Cap LOC estourado para magnitude desproporcionada.

**Exemplo canónico**: P273.15 bbox medido pós-layout — zero demanda
+ custo perf O(N²) + impl walker desproporcional.

#### Razão 3 — Bloqueador estrutural aceito

- Limitação consciente pré-existente documentada em ADR ou L0
  (per ADR-0054 graded).
- Refactor externo fora do escopo (e.g. "Fase 4 candidata
  NÃO-reservada" per política P158).
- DEBT externo fechado mas refactor real adiado.
- Caminho ad-hoc para contornar criaria dívida invisível.

**Exemplo canónico**: P273.16 bbox.y topo-exacto — P156H limitação
+ ADR-0078 §sub-fase (b) scope-out + Caminho 2 ad-hoc cria dívida.

### Como aplicar NO-GO (output mecânico)

1. **Fase A diagnóstico** com inventário factual + análise de risco
   + critério §A.4 (GO) / §A.5 (NO-GO) explícitos.
2. **Decisão binária** GO ou NO-GO fixada com fundamento literal
   (zero caminhos GO viáveis OR caminho 3 scope-out preserved).
3. **Documento `trabalho-previo-externo.md`** produzido como output
   independente:
   - §1 Pré-requisitos REAIS para reanálise futura GO.
   - §2 Critérios para reabrir como GO futuro.
   - §3 Pendência registada permanente.
   - §4 Sub-padrão "Scope-out reconfirmado por Fase A" cumulativo.
4. **ADR-0091 anotação cumulativa** (ou ADR cluster relevante)
   documentando NO-GO + razões + trabalho prévio externo.
5. **L0 anotação** se aplicável (cluster L0 entity).
6. **Relatório passo** com status **SCOPE-OUT-RECONFIRMED** (não
   IMPLEMENTADO; não falha).
7. **Zero alterações código** L1/L3.
8. **Tests workspace preserved** bit-exact.

### Quando NÃO aplicar NO-GO

Esta ADR **não autoriza** NO-GO ad-hoc sem trabalho de diagnóstico:

- **Saltar passo sem documentação**: rejeitado. Pendência fica sem
  trabalho de diagnóstico documentado.
- **NO-GO sem critério Fase A**: rejeitado. Critério §A.4/§A.5
  obrigatório para decisão legítima.
- **NO-GO para evitar trabalho de materialização viável**: rejeitado.
  Se nenhuma das 3 razões legítimas se aplica, GO é o caminho
  correcto.

---

## Análise pureza paridade ADR-0029

**N/A — decisão metodológica.** Pureza preservada (zero código).

---

## Consequências

### Positivas

- **NO-GO é cumprimento honesto** — não falha. Distingue passo
  executado correctamente de passo skip.
- **Output documental legítimo** — `trabalho-previo-externo.md`
  preserva pré-requisitos para reanálise futura.
- **3 razões distintas cobertas** — sub-padrão captura externa /
  interna / estrutural sem overlap.
- **Cluster avança** sem materializar refinos sem demanda ou com
  bloqueador.
- **Disciplina anti-over-engineering** per ADR-0054 graded.

### Negativas

- **Pendência preserved** como item formal aberto — não é
  fechamento definitivo.
- **Reanálise futura GO requer trabalho externo** (pré-requisitos
  §1 do `trabalho-previo-externo.md`).

### Neutras

- Padrão limita-se a "refinos qualitativos opcionais" — features
  user-facing core ou bugs concretos seguem caminhos diferentes
  (GO obrigatório com magnitude apropriada).

---

## Alternativas consideradas

| Alternativa | Prós | Contras | Decisão |
|---|---|---|---|
| **NO-GO output documental (esta ADR)** | Honesto; documenta pré-requisitos; cumprimento crítério | Pendência preserved | **Escolhido** (N=3 empírico) |
| Materializar mesmo sem demanda/com bloqueador | Sem pendência | Over-engineering; risco regressão; dívida ad-hoc | Rejeitado per ADR-0054 graded |
| Saltar passo (skip) | Rápido | Sem trabalho de diagnóstico documentado; pendência sem pré-requisitos | Rejeitado (transparência perdida) |
| NO-GO ad-hoc sem Fase A formal | Magnitude XS | Sem critério rigoroso; passo ad-hoc não-auditável | Rejeitado (rigor perdido) |

---

## Precedentes citáveis

**3 aplicações empíricas cumulativas**:

- **P273.14** (2026-05-18) — Razão **externa** (CMYK-ICC profile
  licensing + crate + invariante L0). 3 pré-requisitos identificados.
  Sub-padrão **inaugural N=1**.
- **P273.15** (2026-05-18) — Razão **interna** (Bbox medido: zero
  demanda + custo perf O(N²)). 2 pré-requisitos. Reaplicação **N=2**.
- **P273.16** (2026-05-18) — Razão **estrutural aceita** (Bbox.y:
  P156H + ADR-0078 §sub-fase b; descoberta empírica DEBT-56 fechado
  actualiza premissa spec). 3 pré-requisitos. **N=3 crossing limiar
  formalização**.

**Documentos cross-reference**:
- `00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`.
- `00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`.
- `00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`.
- `00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md` §A.0 —
  descoberta empírica que actualiza premissa spec (caso onde Fase A
  factual prevalece sobre premissa documental).
- ADR-0091 §"Anotação cumulativa P273.14/15/16" — registo formal
  pre-P273.17.
- ADR-0054 — graded — fundamento meta-decisional aceitando outputs
  documentais.

---

## Próximos passos

- **Aplicação a outras pendências classificadas como refino
  qualitativo opcional**: candidatos pós-cluster Gradient incluem
  `P273.X-bis-bbox-medido-pos-layout` (reabrir se demanda surgir),
  refinos análogos em outros clusters.
- **Reaplicação cross-cluster futura** consolidará sub-padrão para
  N=4+ cumulativo.
- **Pendência meta-meta**: padrão "Fase A factual prevalece sobre
  premissa documental" (caso P273.16 descoberta empírica DEBT-56
  fechado) — candidato sub-padrão futuro se reaplicar. NÃO
  formalizado nesta ADR.

---

## Critério de revisão

ADR-0097 será revisada apenas se:
- Padrão NO-GO mostrar abusos sistémicos (e.g. NO-GO usado para
  evitar trabalho viável repetidamente).
- Alternativa cross-cluster surgir com vantagens demonstráveis.

Caso contrário, padrão preserved como **paradigma decisional Fase A
legítimo**.

---

## Relação com sub-padrão "passo administrativo XS/S criar ADRs meta"

P273.17 cria ADR-0095 + ADR-0096 + ADR-0097 simultaneamente — terceira
aplicação de **"passo administrativo XS/S criar/promover ADRs meta"**:
- N=1 P156K (ADR-0064 + 0065).
- N=2 P271 (ADR-0093 + 0094).
- **N=3 P273.17** (ADR-0095 + 0096 + 0097).

Sub-padrão atinge limiar formalização N=3-4 mas **NÃO formalizado
nesta ADR** per anti-padrão over-formalização explícito P273.17 §0.
Documentado em `typst-cluster-gradient-reflexao.md` §7 reflexão
meta-meta para futuro hipotético se N=4 surgir.

---

*ADR-0097 imutável produzido em 2026-05-18 como output legítimo
do passo administrativo P273.17. Sub-padrão empírico "Scope-out
reconfirmado por Fase A" N=3 cumulativo crossing limiar formalização
N=3-4 — paradigma decisional Fase A consolidado com 3 razões NO-GO
distintas e legítimas (externa P273.14 + interna P273.15 + estrutural
aceita P273.16).*
