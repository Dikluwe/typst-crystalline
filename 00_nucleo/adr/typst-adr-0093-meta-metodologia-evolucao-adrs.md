# ⚖️ ADR-0093: Meta-metodologia de evolução de ADRs (scope-out revogado parcialmente + Anotação cumulativa)

**Status**: `EM VIGOR` (criada directamente; paridade pattern P260
ADR-0084/0085 e P268.1 ADR-0090)
**Data**: 2026-05-17
**Autor**: Humano + IA
**Validado**: Passo 271 — formaliza 2 sub-padrões empíricos N≥6
cumulativos sobre como ADRs evoluem em cristalino.
**Diagnóstico prévio**: inline em P271 §1.A + §1.B (paridade
auto-aplicação ADR-0065 inline — passo administrativo XS).
**Passo origem**: P271 (passo administrativo XS meta-formalização)
**Cluster**: Metodologia / ADRs / Documentação
**Tipo**: meta-ADR formalizando 2 sub-padrões empíricos N≥6 cumulativos

---

## Contexto

Cluster Gradient série P270 (5 sub-passos: P270/P270.1/P270.2/
P270.3/P270.4) produziu 2 sub-padrões metodológicos sobre evolução
de ADRs que atingiram limiar formalização clara N≥6:

1. **"ADR scope-out revogado parcialmente"** N=6 cumulativo —
   P267/P269/P270/P270.2/P270.3/P270.4.
2. **"Anotação cumulativa em vez de ADR nova"** N=10 cumulativo
   consolidação clara — P258.B/P259.B/P263/P265/P268/P268.2/P270/
   P270.1/P270.2/P270.4.

Ambos sub-padrões são sobre **como ADRs evoluem** ao longo do tempo
em cristalino: quando criar nova, quando anotar, quando revogar
parcialmente. Empiria já estabelecida; meta-ADR formaliza para
referência cross-cluster futura.

**Marco metodológico P271**: cluster Gradient série P270 produziu
cluster metodológico — sub-padrões formalizados meta-ADRs paridade
P260 (ADR-0084/0085) e P268.1 (ADR-0090).

---

## Decisão — Pattern 1: ADR scope-out revogado parcialmente

**Quando aplicar**:

- ADR existente tem §"scope-outs" listando elementos não-materializados.
- Passo futuro materializa um (ou alguns) desses elementos sem
  invalidar decisão de fundo da ADR.
- Elementos revogados são localmente identificáveis (não revogam ADR
  inteira).

**Como aplicar**:

1. Anotar §"scope-outs" da ADR-alvo: elementos revogados marcados
   `revogado P<passo>` ou `~~strikethrough~~`.
2. Status da ADR-alvo preservado (`EM VIGOR` ou `IMPLEMENTADO`).
3. Adicionar §"Anotação cumulativa P<passo>" na ADR-alvo documentando
   revogação parcial.
4. Cross-reference em ADR nova (se criada) ou nas notas do passo.

**Quando NÃO aplicar**:

- Revogação invalida decisão de fundo: usar status `REVOGADO` e criar
  ADR substituta.
- Mudança expande decisão original significativamente: criar ADR nova
  com cross-reference (não anotar).

---

## Decisão — Pattern 2: Anotação cumulativa em vez de ADR nova

**Quando aplicar**:

- Actualização significativa a ADR existente sem mudar decisão de
  fundo.
- Refino paramétrico, materialização condicional, ou extensão local.
- Decisão é "como" evolui, não "se" evolui.

**Como aplicar**:

1. Adicionar §"Anotação cumulativa P<passo>" no fim da ADR-alvo.
2. Status preserved (`EM VIGOR` ou `IMPLEMENTADO`).
3. Conteúdo essencial: motivo, alteração estrutural, helpers
   reutilizados, defaults preservados, sub-padrões aplicados.
4. Cross-reference em README §"passos-chave" da entrada do passo.

**Quando NÃO aplicar**:

- Decisão arquitectural distinta surge: criar ADR nova.
- Anotação tornaria-se mais larga que a ADR original: criar ADR nova.
- Múltiplas ADRs precisam anotação simultânea coerente: avaliar
  sub-padrão "Anotação cumulativa cross-ADR" (N≥3 cumulativa via
  P270/P270.1/P270.2/P270.3/P270.4).

---

## Decisão complementar — Anotação cumulativa cross-ADR

Sub-padrão derivado: passo único pode anotar múltiplas ADRs
simultaneamente quando alteração afecta cluster inteiro. Aplicado em
P270/P270.1/P270.2/P270.3/P270.4 (5 aplicações cumulativas; 6 ADRs
em P270 inaugural; 5 ADRs em P270.4 final). Não requer meta-ADR
separada — sub-caso de Pattern 2.

**Critério revisão complementar**: sub-padrão cross-ADR atingir N≥5
formaliza-se candidato meta-ADR dedicada ADR-0095 (não criada P271 —
N=5 já cumprido em P270/P270.1/P270.2/P270.3/P270.4 mas considera-se
sub-caso suficientemente coberto por Pattern 2 nesta ADR).

---

## Histórico empírico

### Sub-padrão "ADR scope-out revogado parcialmente" (N=6)

| N | Passo | ADR alvo | Elemento revogado |
|---|---|---|---|
| 1 | P267 | ADR-0088 §Conic | Conic L1+stdlib materializado (Conic sai do scope-out variants não materializados) |
| 2 | P269 | ADR-0088 §focal_* | focal_center + focal_radius materializados |
| 3 | P270 | ADR-0083 §ColorSpace runtime | ColorSpace runtime L1+stdlib activado |
| 4 | P270.2 | ADR-0083 §DeviceCMYK PDF | Linear+Radial CMYK L3 emit directo |
| 5 | P270.3 | ADR-0090 §Type 6 Coons | Type 6 Coons como estratégia adicional Conic (Type 7 preserved scope-out) |
| 6 | P270.4 | ADR-0083 §DeviceCMYK PDF + ADR-0091 §Conic CMYK | Revogação final absoluta (Conic CMYK activado) |

Limiar formalização clara N=4 ultrapassado em P270.2; pattern
estabelecido sólido por N=6 P270.4.

### Sub-padrão "Anotação cumulativa em vez de ADR nova" (N=10)

| N | Passo | ADR-alvo | Motivo |
|---|---|---|---|
| 1 | P258.B | ADR-0070 style_chain | Refino sem revogar |
| 2 | P259.B | ADR-0073 geometry | Anotação Visualize |
| 3 | P263 | ADR-0087 Linear | PDF emit anotado |
| 4 | P265 | ADR-0088 Radial | PDF emit anotado |
| 5 | P268 | ADR-0089 Conic | PDF emit anotado |
| 6 | P268.2 | ADR-0089 Conic | Refino adaptive N |
| 7 | P270 | ADR-0083 + 5 ADRs cluster Gradient | Anotação cross-ADR (sub-padrão "Anotação cumulativa cross-ADR" inaugural) |
| 8 | P270.1 | ADR-0091 + ADR-0087/0088/0089/0090 | L3 multi-space refino |
| 9 | P270.2 | ADR-0091 + ADR-0083 + ADR-0087/0088/0089/0090 | CMYK directo Linear+Radial |
| 10 | P270.4 | ADR-0092 + ADR-0091 + ADR-0083 + ADR-0089 + ADR-0054 | Coons CMYK activação |

Consolidação clara N=10. Pattern estabelecido como mecanismo principal
de evolução documental ADRs.

---

## Consequências

- ✅ Mecanismo documentado preserva linhagem cumulativa ADRs.
- ✅ Reduz proliferação ADRs (10 anotações cumulativas em série P270
  vs 10 ADRs novas hipotéticas).
- ✅ Pattern recuperação via histórico empírico cross-cluster.
- ⚠️ Risco proliferação anotações cumulativas: salvaguarda via §"Quando
  NÃO aplicar".

---

## Alternativas consideradas

- **Status REVOGADO sempre que scope-out muda**: rejeitada — invalida
  ADRs que ainda têm decisão de fundo válida (e.g. ADR-0090 Type 4
  RGB preserved após Type 6 sair scope-out).
- **ADR nova sempre que actualização significativa**: rejeitada —
  proliferação documental (cluster Gradient teria ~88 ADRs em vez
  das actuais 79; ADRs perderiam coerência).
- **Sub-padrões não formalizar**: rejeitada — N≥6 e N=10 muito
  ultrapassam limiar; meta-formalização para referência futura é
  factualmente justificada.

---

## Critério revisão

Esta ADR pode ser revisitada se:

- Sub-padrão "ADR scope-out revogado parcialmente" começar a
  invalidar decisões de fundo (deveria forçar REVOGADO; ADR-0093
  inválida).
- Anotações cumulativas tornarem-se larger que ADRs originais
  (forçaria criar ADRs novas; ADR-0093 inválida).
- Sub-padrão cross-ADR atingir N≥5 com complexidade própria
  distinta de Pattern 2: candidato meta-formalização dedicada
  ADR-0095.

---

## Subpadrões aplicados

- **Passo administrativo XS criar/promover ADR**: N=5 → **N=6
  cumulativo** (P156K/P160A/P229/P254/P268.1/**P271**).
- **Auto-aplicação ADR-0065 inline**: N=15 → N=16 cumulativo
  (diagnóstico empírico embebido §"Histórico empírico" desta ADR;
  sem ficheiro filesystem novo).
- **Meta-formalização sub-padrões empíricos N≥4**: N=2 → **N=3
  cumulativo** (P260 ADR-0084/0085 + P268.1 ADR-0090 + **P271
  ADR-0093/0094**).

---

## Referências

- ADR-0084 + ADR-0085 (formalização P260; precedente pattern
  meta-ADR EM VIGOR criada directa).
- ADR-0090 (formalização P268.1; precedente pattern meta-ADR EM
  VIGOR criada directa).
- ADR-0094 (companheira P271; meta-operacional specs).
- ADR-0083, ADR-0090, ADR-0091, ADR-0092 (ADRs instanciadoras dos
  sub-padrões; anotadas cumulativa P271).
- ADR-0054 (perfil graded DEBT-1; reforçado P271 via mecanismos
  operacionais).
- Passos cumulativos §"Histórico empírico" desta ADR (P258.B,
  P259.B, P263, P265, P267, P268, P268.2, P269, P270, P270.1,
  P270.2, P270.3, P270.4).
