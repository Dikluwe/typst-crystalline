# Relatório do passo P202

**Data de execução**: 2026-05-05.
**Executor**: Claude Code (LLM externa, modelo Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-202.md`.
**Natureza**: passo L0-puro / administrativo. Zero código.
Zero testes. Zero ADRs criadas. Zero DEBTs criados.

---

## §1 O que foi feito

P202 produziu três outputs documentais conforme spec §5
mais um backup conceptual:

### Output 0 — backup do snapshot anterior

`00_nucleo/snapshot-2026-05-05.pre-P202.md` (reconstrução
conceptual). O snapshot anterior nunca existiu como ficheiro
discreto; os valores estavam dispersos por:
- P192 consolidado §10.
- P192B spec.
- ADR-0070 (tabela validação empírica).
- Spec P201 §1.

O backup é uma reconstrução agregadora dessas fontes,
preservando a forma conceptual do snapshot pré-P202 sem
modificar os ficheiros originais.

### Output 1 — `00_nucleo/snapshot-2026-05-05.md` (reescrito)

Snapshot reconciliado com realidade empírica. Estrutura
§1-§13:
- §1 Identificação.
- §2 Cobertura empírica (tests 1823, lint 0, comemo 0.4).
- §3 Marcos arquitectónicos (M1-M7, M9, F1, F3 parcial; M8
  pendente).
- §4 ADRs no ciclo M5/M6/M7 (6 ACEITES, 2 EM VIGOR,
  3 PROPOSTAS; correcção retroactiva ADR-0067).
- §5 Estado estrutural pós-P192B (verificado empíricamente).
- §6 Activado vs preservado vs pendente para M8.
- §7 Lacunas (#1, #1b residuais; #2 parcial; #3-#7
  fechadas).
- §8 M8 escopo (recalibrado: 22 fields, 1823 tests,
  6 ACEITES).
- §9 Caminhos ortogonais.
- §10 Convenções e padrões consolidados.
- §11 Métricas cumulativas.
- §12 Diferenças face à versão pré-P202.
- §13 Resumo para abrir nova sessão.

### Output 2 — `00_nucleo/diagnosticos/typst-passo-202-reconciliacao.md`

Registo de reconciliação com:
- §1 Tabela resumo (questão → decisão → alteração).
- §2-§10 Detalhe por questão (D1, D2, D7, D11, N8.1-N8.5).
- §11 Lista de itens fora de escopo.
- §12 Critério de progressão verificado.
- §13 Lista resumida.

### Output 3 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:
- Leitura do spec P202 e contexto da auditoria delta:
  ~5 min.
- Localização do snapshot (descobrindo que era conceptual):
  ~10 min.
- Validação empírica restante (ElementPayload, ElementKind,
  Content, LayouterRuntimeState, helpers ADR-0069):
  ~5 min.
- Investigação ADR-0067 (status PROPOSTO vs referências
  ACEITE em consolidados): ~5 min.
- Redacção do backup conceptual (Output 0): ~5 min.
- Redacção do snapshot reconciliado (Output 1): ~15 min
  (output ~12 KB).
- Redacção do registo de reconciliação (Output 2):
  ~10 min (output ~13 KB).
- Redacção deste relatório: ~5 min.

**Total**: ~60 min de sessão.

---

## §3 Decisões tomadas durante a leitura

### 3.1 Localização do snapshot (C1)

**Descoberta crítica**: o "snapshot 2026-05-05" referido pela
spec P201 §1 e P202 §C1 **não existia como ficheiro discreto**.
Os valores ("1.802 tests", "7 ADRs ACEITES", "Layouter 19
fields", "33 aplicações diag-1º") estavam dispersos por:

- `00_nucleo/materialization/typst-passo-192-relatorio-consolidado.md` §10.
- `00_nucleo/materialization/typst-passo-192b.md` (cláusulas
  numéricas).
- `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md` (tabela validação empírica: "Layouter `counter` field | 19/20 fields").
- `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md`.
- Spec P201 §1 (precondição declarada).

**Decisão**: criar
`00_nucleo/snapshot-2026-05-05.pre-P202.md` como backup
agregador (reconstrução conceptual a partir das fontes) e
escrever o novo snapshot em
`00_nucleo/snapshot-2026-05-05.md`.

**Implicação**: a "regra byte-idêntica" do backup
(spec C1.3) não se aplica literalmente — é-o aplicada à
forma agregadora; os ficheiros fonte permanecem intactos
como autoridade canónica original. Documentado em §12 do
registo de reconciliação.

### 3.2 Espaço C4 — variants encontrados vs spec esperado

Spec C4 indicava valores que se revelaram incorrectos:

| Item | Spec C4 esperava | Empírico | Decisão |
|------|:---:|:---:|---|
| ElementPayload variants | 13 | **12** | Reportar valor empírico no snapshot |
| ElementKind variants | 10 | **10** | ✅ confirmado |
| Content enum variants | 13 | **60** | Reportar valor empírico (spec C4 estava substancialmente errada) |
| LayouterRuntimeState fields | 3 | **3** | ✅ confirmado |
| Helpers ADR-0069 | "3 + 1 eliminado" | **3 activos + 1 eliminado** | ✅ confirmado |

**Decisão**: registar valores empíricos. **Não corrigir
a spec P202** — o seu propósito era validar empíricamente,
e essa validação revelou os valores reais. Documentar
divergência em §5 do snapshot e §1/§5 do registo de
reconciliação.

### 3.3 ADR-0067 — manter PROPOSTO (C8)

**Achado**: 5 fontes nos consolidados P190/P192 listam
ADR-0067 como ACEITE; o ficheiro canónico
`00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md`
linha 3 declara `Status: PROPOSTO`. As 4 condições do
plano de validação (linha 240) **não foram cumpridas**.

**Decisão**: manter PROPOSTO no snapshot reescrito;
documentar correcção retroactiva.

**Justificação**: spec C8 explicitamente declara: "P202
não promove ADRs unilateralmente. Apenas regista o que
está e regista a decisão." Promoção fica para P203+ se
for o caminho.

### 3.4 ADR-0063 — não criar ficheiro (C9)

**Decisão**: apenas documentar slot vazio; **não criar
`typst-adr-0063-RESERVADO.md`**.

**Justificação**: spec C9 recomenda explicitamente "apenas
documentar". Spec §8 não-objectivos confirma:
"P202 não cria ADRs novas (ADR-0063 não é criado mesmo
que C9 fosse pela alternativa de formalização)."

### 3.5 N8.3, N8.5 — não modificar relatórios anteriores

**Decisão**: **aceitar ausência** de datação P157-P160
(N8.3) e **aceitar absorção** pelos consolidados (N8.5).

**Justificação**: spec C10 e C12 recomendam essas
alternativas explicitamente para preservar histórico.
Modificação retroactiva quebraria a regra de preservação.

### 3.6 N8.4 — aceitar hipótese e documentar

**Decisão**: aceitar a distinção "ordem de publicação ≠
ordem de dependência semântica" e documentar; não
investigar Git log.

**Justificação**: spec C11 recomenda esta alternativa.
Investigação Git log fica fora do escopo administrativo
de P202.

### 3.7 Snapshot §3 marcos — ordem semântica

**Decisão**: listar marcos no snapshot §3 pela ordem de
**dependência semântica** (M5 universal P200B → M6 P190I
→ M7 P192B), não pela ordem de **datação dos relatórios**.

**Justificação**: P201 auditoria delta §8.4 documentou
a anomalia. Snapshot reescrito reflecte a estrutura real;
auditoria delta absorve o detalhe da anomalia.

---

## §4 Sugestões para o próximo passo (não-vinculativas)

Per spec §6, P202 não decide. Quatro caminhos
não-vinculativos:

### 4.1 Caminho A — P203A diagnóstico-primeiro de M8

Caminho default. Aplicaria pattern diagnóstico-primeiro
consolidado (~35 aplicações) ao M8: substituir hash-based
convergence por comemo nativo.

Pré-condição cumprida (per snapshot §8): M5+M6+M7
estruturalmente fechados. Magnitude esperada L
cross-modular.

### 4.2 Caminho B — P203 administrativo: promoção formal de ADR-0067

Se humano considerar relevante, P203 pode validar as 4
condições do plano de validação ADR-0067 e promover a
IMPLEMENTADO formal. Não é pré-requisito de M8.

Verificação a fazer:
1. Pelo menos uma propriedade alvo materializada com
   pattern attribute-grammar.
2. Walk recebe parâmetro de atributos herdados sem
   violar P163.
3. Tests E2E confirmam scoping léxico.
4. Comparação com Caminho 2 hipotético.

Trabalho provavelmente M-L (não trivial); pode justificar
sub-passo de feature dedicado para materializar
`numbering_active` com pattern.

### 4.3 Caminho C — P203 lacunas residuais

Lacunas #1 (Position) e #1b residuais. P203 dedicado pode
ser apropriado se M8 depender criticamente de Position
concreto.

Per snapshot §7: "potencial bloqueio para queries
location-aware completas". Auditor humano deve avaliar.

### 4.4 Caminho D — Pausa estratégica

Consolidar marcos fechados antes de M8. Validar saída
cristalino vs vanilla em corpus de paridade actual (sem
comemo) para identificar gaps remanescentes.

---

## §5 Critério de progressão (verificação)

Per spec §6, P202 está concluído quando:

- [x] Os 3 ficheiros existem:
  - `00_nucleo/snapshot-2026-05-05.md` (Output 1).
  - `00_nucleo/diagnosticos/typst-passo-202-reconciliacao.md`
    (Output 2).
  - `00_nucleo/materialization/typst-passo-202-relatorio.md`
    (Output 3).
- [x] Backup do snapshot anterior existe:
  `00_nucleo/snapshot-2026-05-05.pre-P202.md` (reconstrução
  conceptual).
- [x] C1-C12 todos endereçados:
  - C1: localização identificada (snapshot conceptual);
    backup criado.
  - C2: §3 marcos reescritos com sub-passos finais reais.
  - C3: §4 ADRs reescrita com 6 ACEITES + 2 EM VIGOR +
    3 PROPOSTAS.
  - C4: §5 estrutural reescrita com 22 fields Layouter,
    9 sub-stores, 12 ElementPayload variants, 60 Content
    variants, etc.
  - C5: §11 métricas com 1823 tests, 70 ADRs, ~35 diag-1º.
  - C6: §7 lacunas com #1/#1b residuais, #2 parcial,
    #3-#7 fechadas.
  - C7: §8 M8 escopo recalibrado.
  - C8: ADR-0067 mantido PROPOSTO; correcção retroactiva
    registada.
  - C9: ADR-0063 slot vazio documentado; ficheiro não
    criado.
  - C10: datação P157-P160 — ausência aceite.
  - C11: anomalia P190 vs P195 — distinção aceite e
    documentada.
  - C12: relatórios individuais ausentes — convenção de
    absorção formalizada.
- [x] 4 divergências (D1, D2, D7, D11) reflectidas no
  snapshot.
- [x] 5 notas de auditor (N8.1-N8.5) com decisão
  registada.
- [x] Snapshot internamente consistente
  (§3↔§4↔§5↔§11 verificado cruzadamente).

---

## §6 Não-objectivos respeitados

Per spec §8, P202 não:

- [x] Não decidiu o caminho de M8 (apenas listou opções
  em §4 deste relatório).
- [x] Não propôs ADR-0073.
- [x] Não tocou em código.
- [x] Não promoveu ADRs (ADR-0067 mantém PROPOSTO).
- [x] Não criou ADRs novas (ADR-0063 não criado).
- [x] Não modificou relatórios de passos anteriores
  retroactivamente.
- [x] Não pré-definiu sub-passos de M8.

---

## §7 Notas operacionais

### 7.1 Acesso à pasta `context/`

Spec P202 §3 autoriza leitura de `00_nucleo/context/`
**apenas se for esse o local do snapshot anterior**.
Verificação revelou que **não existe ficheiro de snapshot
em `context/`**. Pasta não foi acedida fora dessa
verificação inicial.

### 7.2 Verificação cruzada do snapshot

Snapshot reescrito foi verificado para consistência
interna:

- §3 (marcos) ↔ §4 (ADRs): cada marco fechado tem ADR
  correspondente em ACEITE listada.
- §5 (estrutural) ↔ §11 (métricas): valores numéricos
  consistentes (Layouter 22; sub-stores 9; trait 20;
  walk 7; loops 2; ElementPayload 12; ElementKind 10;
  Content 60; LayouterRuntimeState 3; tests 1823).
- §7 (lacunas) ↔ §3 (marcos): #3 fechada P200B coincide
  com M5 universal P200B.
- §12 (diferenças face pré-P202) lista todos os pontos
  reconciliados.

### 7.3 Trust but verify do snapshot anterior

A reconstrução conceptual (Output 0) baseia-se em
citações directas das fontes (P192 consolidado §10,
P192B spec, ADR-0070). Cada valor declarado tem
referência explícita à fonte. Auditor humano pode
recompilar o pré-P202 a partir das fontes originais
e verificar.

### 7.4 Estrutura §1-§13 do snapshot

A spec P202 referia "§1-§13 do original". Como o
original não existia como ficheiro discreto, a
estrutura §1-§13 foi inferida como a estrutura
canónica que faria sentido para o snapshot:

- §1 identificação;
- §2 cobertura;
- §3 marcos;
- §4 ADRs;
- §5 estrutural;
- §6 activado/preservado/pendente;
- §7 lacunas;
- §8 M8 escopo;
- §9 ortogonais;
- §10 convenções;
- §11 métricas;
- §12 diferenças face pré-P202;
- §13 resumo para nova sessão.

§12 e §13 são **adições** que P202 introduz para
suportar a transição (não há contraparte directa no
material pré-P202; servem para auditoria e abertura
de sessão).

---

**Fim do relatório P202.**
