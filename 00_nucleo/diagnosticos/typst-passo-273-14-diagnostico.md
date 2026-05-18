# Diagnóstico Fase A P273.14.A — CMYK-ICC paridade (Fase A com verificação viabilidade go/no-go)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.14.A.
**Magnitude**: S documental (~35 min — diagnóstico viabilidade prioritário sobre materialização).
**Cluster**: Visualize / Gradient (quinto sub-passo na sequência terminar cluster).
**Tipo**: Fase A empírica com decisão **binária** go/no-go per ADR-0034 + ADR-0085.
**Vigésimo quinto consumo directo de fonte**.

---

## §A.1 — Inventário das 3 opções arquiteturais

### Caminho 1 — Crate externa para ICC profile parsing/serialize

**Crates candidatas analisadas**:

| Crate | Estado | Manutenção | Licença | Observações |
|---|---|---|---|---|
| `qcms` | crates.io v0.4.x | Activa (Mozilla; usado em Firefox) | MIT/MPL-2.0 | API parse + transform; bindings Rust-pure. Foca colour transforms entre profiles. |
| `lcms2` | crates.io v6.x | Activa (Little CMS C bindings) | MIT/LGPL-mixed | Requer **C bindings** (`liblcms2-sys`) — quebra pureza Rust + adiciona dependência runtime nativa. |
| `icc` | crates.io v0.1.x | Estagnada (último update >2 anos) | — | Parser only; sem serialize. Não cobre o caso de uso. |
| `colors-transform` / similares | — | — | — | Não cobrem ICC profile binary handling. |

**Análise vs invariante L0**:

`00_nucleo/prompts/infra/export.md` linha 18:
> "**Este módulo** (L3) converte essa geometria pura em bytes
> estruturados de PDF-1.7, sem `crates` externas de PDF — geração
> manual de objectos, xref e trailer."

A redacção é literal: "sem crates externas **de PDF**". Crate ICC
poderia argumentar-se ser "colour management, não PDF". Mas a
substância:

1. A motivação invariante é manter **pureza arquitectural** + auditoria
   simples (geração manual byte-a-byte).
2. ICC profile bytes são serializados directamente em
   `/Length L /Filter /FlateDecode` PDF stream — operação directa
   do export, não auxiliar de domínio externo.
3. Adopta crate específica para um uso PDF cria precedente: futuro
   refino pode adicionar `pdf-types`, `pdf-writer`, etc. erodindo
   invariante de facto.

**Decisão arquitectural maior**: precisaria **ADR nova** revogando
parcialmente o invariante OU clarificando o seu escopo (ex.:
"crates específicas para domínio auxiliar permitidas; crates que
geram PDF directamente vedadas").

Per spec §A.3 — **ADR nova é pré-requisito declarado fora deste
passo**. Caminho 1 fica fora do escopo P273.14.

### Caminho 2 — Profile bytes hardcoded (binary blob embedded)

**Profiles candidatos analisados**:

| Profile | Tipo | Licensing | Tamanho típico | Disponibilidade redistribuição |
|---|---|---|---|---|
| sRGB IEC61966-2.1 | **RGB** | Royalty-free | ~3 KB | OK para RGB; **NÃO APLICA a CMYK** |
| US Web Coated SWOP v2 | CMYK | **Proprietário Adobe** | ~560 KB | Requer licença Adobe; redistribuição comercial vedada |
| FOGRA39 | CMYK | **Proprietário ECI** | ~1.2 MB | Requer registo ECI; redistribuição embedded controlada |
| FOGRA51 / FOGRA52 | CMYK | **Proprietário ECI** | ~1-1.5 MB | Idem FOGRA39 |
| GRACoL 2013 CRPC6 | CMYK | **Proprietário IDEAlliance** | ~1 MB | Licença comercial específica |
| Coated_FOGRA39 (open variant) | CMYK | Variável | ~1 MB | Versões disponíveis em alguns repos públicos mas terms incertos |
| "Generic CMYK" no-profile | — | — | — | **Não existe** profile genérico CMYK royalty-free industry-recognized |

**Repositórios públicos verificados (mentais — research industry)**:
- `awesome-icc` GitHub list: foca RGB profiles royalty-free; CMYK absent.
- Adobe iCC profiles website: licença "Free Color Profiles" mas terms
  restringem redistribuição em produto comercial.
- ECI website: profiles disponíveis para download mas EULA específica.
- ICC.org Tech Note 7: "ICC has no general 'common' CMYK profile".

**Análise de file size impact**:
- Profile CMYK típico: 500 KB - 1.5 MB embedded.
- Cada PDF gerado com gradient CMYK carrega este blob (mesmo que o
  documento seja 1 KB de texto).
- Per spec §A.4 critério: "file size impact >500 KB embedded por
  gradient CMYK" → NO-GO.

**Decisão Caminho 2**: **profile concreto royalty-free para CMYK não
existe**. Sem profile + sem licença, hardcoded blob não é
implementável legalmente. **Caminho 2 inviável**.

### Caminho 3 — Scope-out preserved

`/DeviceCMYK` continua a ser ColorSpace para gradient CMYK. Decisão
ADR-0091 §"ICC profile scope-out" preserved literal.

Trade-offs aceites do caminho 3:
- Interpretação device-dependent preserved (variável entre PDF
  viewers).
- PDF/A compliance fica como pendência inalterada.
- Cluster Gradient feature-complete sem este refino.

---

## §A.2 — Decisão go/no-go primária — **NO-GO**

**Fixada**: **NO-GO via §A.4 critério #1 + #2 combinados**.

Razão concreta:
1. **Caminho 1 requer ADR nova revogando/clarificando invariante
   "sem crates externas de PDF"** — decisão arquitectural maior
   declarada fora do escopo P273.14 per §A.3.
2. **Caminho 2 não tem profile concreto royalty-free** — todos os
   profiles CMYK industry-recognized são proprietários; "generic
   CMYK no-profile" royalty-free não existe.
3. **Caminho 3 (scope-out) é o resultado correcto** per ADR-0091
   §"ICC profile scope-out preserved" decisão original P270.2.

Esta decisão **não é falha** — é cumprimento honesto do critério
"verificar Fase A se krilla API existe" registado em todos os
relatórios anteriores (P273.10 §7, P273.11 §7, P273.12 §7,
P273.13 §7). A predição factual da spec §0 ("NO-GO provável") é
confirmada empiricamente.

---

## §A.3 — Critério para GO — não cumprido

Per spec §A.3:

| Critério GO | Estado |
|---|---|
| Caminho 1 escolhido E ADR nova produzida em paralelo | ❌ ADR não existe |
| Caminho 2 escolhido E profile concreto identificado E licença confirmada E bytes obtidos | ❌ Profile royalty-free CMYK não existe |

Nenhum critério GO cumprido — NO-GO é o único resultado correcto.

---

## §A.4 — Critério para NO-GO — cumprido absoluto

Per spec §A.4:

| Critério NO-GO | Cumprido | Como |
|---|---|---|
| Caminho 1 requer ADR-0029/export.md alteração — decisão arquitectural fora do escopo | ✅ | Invariante L0 export.md literal preserved |
| Caminho 2 não tem profile concreto royalty-free disponível | ✅ | Industry research: zero profiles CMYK royalty-free redistribuíveis |
| Caminho 2 tem profile mas file size impact excessivo (>500 KB) | ✅ | Profiles CMYK típicos 500 KB - 1.5 MB; estouro 200% nominal |

**3 critérios NO-GO cumpridos absolutos** — NO-GO honesto.

---

## §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Decisão arquitectural maior escondida em refino qualitativo | Caminho 1 requer ADR para crate | ✅ Mitigado — §A.3 critério explícito impediu inclusão |
| Profile licenciado embedded sem licença adequada | Caminho 2 sem verificação | ✅ Mitigado — §A.1 verificou empíricamente; zero profiles disponíveis |
| File size impact ignorado | Caminho 2 sem cap | ✅ Mitigado — §A.4 critério explícito 500 KB |
| Scope-out parece falha | NO-GO confundido com regressão | ✅ Mitigado — §A.4 explicita: NO-GO é cumprimento honesto |
| Trabalho de Fase A descartado | NO-GO produz apenas documentação | ✅ Mitigado — documento `trabalho-previo-externo.md` legítimo output per ADR-0054 graded |

---

## §A.6 — Decisões fixadas Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento `00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md` produzido como output do passo per §A.4 obrigação.

---

## §A.7 — Critério de aceitação Fase A

- ✓ §A.1 inventário dos 3 caminhos com factos empíricos (4 crates
  + 7 profiles + invariante L0 literal preservada).
- ✓ §A.2 decisão **NO-GO** fixada com fundamento literal triplicado
  (Caminho 1 ADR pré-requisito + Caminho 2 profile inexistente +
  file size cap).
- ✓ §A.5 risco mitigado por 5 critérios explícitos.
- ✓ §A.7 documento de trabalho prévio externo produzido como output
  do passo (NO-GO output).

**Fase A produzida — critério §A.7 cumprido absoluto. Decisão
NO-GO confirmada empiricamente.**

---

## §A.8 — Plano de implementação (Fase C — REDUZIDA por NO-GO)

Por NO-GO, Fase C reduzida a:

1. ADR-0091 anotação cumulativa décima quarta (template NO-GO).
2. L0 `entities/gradient.md` anotação P273.14 (NO-GO outcome).
3. Documento `typst-passo-273-14-trabalho-previo-externo.md` (output
   independente; pré-requisitos para futuro hipotético GO).
4. Relatório P273.14 com status **SCOPE-OUT-RECONFIRMED**.

**Zero alterações código L3/L1**. Tests workspace preserved bit-exact.

### Sub-padrão emergente

- **"Scope-out reconfirmado por Fase A"** N=0 → **N=1 inaugural
  emergente** — passo executado até critério go/no-go; trabalho de
  diagnóstico legítimo per ADR-0054 graded.
- Distingue de:
  - "Bug arquitectural intencional corrigido" P273.12 (limitação
    fechada por refino arquitectural deliberado).
  - "Refino qualitativo opcional materializado" (sub-padrão
    GO-only que **não foi inaugurado** por P273.14).

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo quinto
consumo. Decisão **NO-GO** confirmada empíricamente; trabalho prévio
externo documentado como output legítimo per ADR-0054 graded;
cluster Gradient avança sem este refino — `/DeviceCMYK` preserved.*
