# Relatório de Levantamento — Passo 1084: Levantamento e Triagem de Candidatos para Fatiamento

**Data**: 2026-08-18  
**Passo**: 1084 — Levantamento e Triagem de Candidatos para Fatiamento  
**Gate**: Nenhum (Levantamento factual e medição de co-mudança — sem fatiamento de código neste passo)  
**Status**: CONCLUÍDO COM ÊXITO (Medição factual de co-mudança histórica `cochange_metrics.py` executada nos 7 maiores candidatos; Critério-Zero aplicado; Lista curta fundamentada pelo Critério 3)

---

## 1. Contexto e Método Canônico de Auditoria

O método de fatiamento (`00_nucleo/prompts/auditar-fatiamento.md` e a emenda `00_nucleo/emenda-auditar-fatiamento-constantes-geometricas.md`) estabelece que o **Critério 3 (Co-mudança Histórica)** é o único que consistentemente produz sinal decisivo para fronteiras reais de fatiamento, enquanto o tamanho bruto é apenas um filtro inicial que pode enganar por defeito ou por excesso (P1006, P1014).

A ordem normativa de avaliação exige:
1. **Passo 0 (Critério-Zero)**: Verificar se o arquivo é interface (`trait`) ou agregado de funções livres.
2. **Passo 1 (Inventário de Símbolos)**: Mapeamento de funções de produção vs funções de teste.
3. **Passo 2 (Critério 3 — Co-mudança Histórica)**: Medição de commits de corpo e clusters de alteração simultânea via `tools/analysis/cochange_metrics.py` (com desconto de ruído de resselo de hash).
4. **Passo 3 (Pureza de Contexto e Anti-padrões de Hub per ADR-0104)**: Identificar se o hub acumula lógica ou apenas abriga testes compartilhados.

---

## 2. Medição Factual de Co-mudança Histórica nos 7 Candidatos

Executada a análise temporal completa via `tools/analysis/cochange_metrics.py` nos 7 maiores arquivos de implementação:

| Candidato | Linhas | Commits Totais | Linhagem (Resselo) | Commits de Corpo | Fns Produção | Fns Teste | Clusters Reais (≥2 fns) | Top Funções com Maior Co-mudança |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :--- |
| **`layout/mod.rs`** | 2.448 | 196 | 11 | **185** | 46 | 93 | **81** | `layout_content` (127), `new` (49), `measure_content_constrained` (39), `finish` (30) |
| **`introspect.rs`** | 4.795 | 124 | 0 | **124** | 12 | 133 | **61** | `walk` (70), `materialize_time` (60), `convert_refs` (5), `compute_labelled` (5) |
| **`eval/rules.rs`** | 2.051 | 130 | 52 | **78** | 30 | 30 | **33** | `extract_pt` (44), `apply_show_rules` (24), `eval_show_rule` (18), `eval_set_rule` (17) |
| **`layout/cursor.rs`** | 1.122 | 86 | 17 | **69** | 27 | 1 | **39** | `new_page` (26), `emit_deferred_float` (23), `flush_pending_footnote_bodies` (20), `flush_line` (19) |
| **`math/layout/mod.rs`** | 1.912 | 64 | 8 | **56** | 42 | 64 | **27** | `layout_node` (18), `offset_item` (15), `layout_sequence` (11), `layout_text_node` (9) |
| **`stdlib/layout.rs`** | 1.943 | 58 | 11 | **47** | 33 | 0 | **31** | `native_grid` (16), `native_block` (13), `native_box` (12), `native_align` (9) |
| **`stdlib/collections.rs`** | 2.139 | 28 | 0 | **28** | 67 | 48 | **19** | `try_dispatch_collection_method` (17), `array_sorted` (7), `str_match` (6) |

---

## 3. Análise Detalhada dos Candidatos Críticos

### 3.1 `layout/mod.rs` — Candidato Prioritário #1 (Forte Sinal de Co-mudança e Fragmentação)
- **Diagnóstico Temporal**: Líder absoluto em atividade de desenvolvimento no compilador (185 commits de corpo e 81 clusters de co-mudança).
- **Estrutura**: Possui 46 métodos de produção distintos operando sobre `Layouter`. O método `layout_content` (127 commits) co-muda frequentemente com medições (`measure_content_constrained`), despacho de sub-frames e pilhas.
- **Passa no Critério-Zero**: Agregado puro (0 traits).

### 3.2 `introspect.rs` — Diagnóstico do Anti-Padrão de Hub vs Acúmulo de Testes (ADR-0104)
- **Diagnóstico Volumétrico vs Estrutural**: Apesar de ser o maior arquivo da base (4.795 linhas), a medição revela que **mais de 70% das linhas pertencem aos 133 testes unitários**.
- **Lógica de Produção**: Possui apenas **12 funções de produção**, concentradas quase inteiramente no eixo monolítico `walk` (70 commits) e `materialize_time` (60 commits).
- **Enquadramento ADR-0104**: `introspect.rs` funciona hoje como um repositório central de testes E2E acoplado a um walker monolítico. Per regra do Passo 4 do método, os testes devem permanecer no hub central, enquanto as lógicas satélites de `walk` (extração de labels, bibliografia, contadores e headings) são as verdadeiras candidatas a fatiamento modular.

### 3.3 `eval/rules.rs` vs `stdlib/collections.rs` (O Tamanho vs a Co-mudança)
- `eval/rules.rs` (2.051 l) teve **78 commits de corpo e 33 clusters**, demonstrando forte acoplamento entre regras `#set` e `#show` que justificam fatiamento por clusters de regras.
- Por outro lado, `stdlib/collections.rs` (2.139 l), apesar de mais longo, teve apenas **28 commits de corpo e 19 clusters**, pois os métodos de `array`, `dict` e `str` são funções puras ortogonais que raramente co-mudam.

### 3.4 `layout/cursor.rs` — Esclarecimento de Precedentes e Avaliação Factual
- **Precedente P1062**: O P1062 tratou exclusivamente de desambiguação de nomes de prompts `.md` (evitar colisões de nomes de arquivos documentais), sem avaliar fatiamento arquitetural por tamanho ou co-mudança.
- **Avaliação Factual**: Com 1.122 linhas, 69 commits de corpo e 39 clusters reais, `cursor.rs` é um agregado denso de gerenciamento de quebra de páginas (`new_page`), notas de rodapé (`flush_pending_footnote_bodies`) e floats diferidos (`emit_deferred_float`), constituindo um candidato legítimo de média/alta prioridade.

---

## 4. Aplicação do Critério-Zero aos Candidatos

| Arquivo Candidato | Linhas | Traits Públicos | Classificação | Veredito |
| :--- | :---: | :---: | :---: | :---: |
| **`compiler/layout/mod.rs`** | 2.448 | 0 | **Agregado Puro** (`impl Layouter`) | **Aprovado (Candidato #1)** |
| **`compiler/introspect.rs`** | 4.795 | 0 | **Agregado + Suite E2E** | **Aprovado (Candidato #2)** |
| **`compiler/eval/rules.rs`** | 2.051 | 0 | **Agregado Puro** (set/show rules) | **Aprovado (Candidato #3)** |
| **`compiler/layout/cursor.rs`** | 1.122 | 0 | **Agregado Puro** (fluxo/cursor) | **Aprovado (Candidato #4)** |
| **`compiler/math/layout/mod.rs`** | 1.912 | 0 | **Agregado Puro** (`MathLayouter`) | **Aprovado (Candidato #5)** |
| **`compiler/stdlib/layout.rs`** | 1.943 | 0 | **Agregado Puro** (nativas de layout) | **Aprovado (Candidato #6)** |
| **`compiler/stdlib/collections.rs`** | 2.139 | 0 | **Agregado Puro** (nativas ortogonais) | **Aprovado (Candidato #7)** |
| *`entities/introspector.rs`* | 1.525 | `pub trait Introspector` | **Interface / Trait** | **Recusado (Critério-Zero)** |
| *`compiler/layout/metrics.rs`* | 90 | `pub trait Metric` | **Interface / Trait** | **Recusado no P1006** |

---

## 5. Lista Curta Reordenada por Co-mudança Histórica (Critério 3)

A priorização rigorosa pelo Critério 3 estabelece o roteiro para os próximos passos de fatiamento:

1. **Candidato #1 — `01_core/src/compiler/layout/mod.rs` (185 commits de corpo, 81 clusters, 46 fns de produção)**:
   - *Justificativa*: Maior densidade de co-mudança e maior concorrência funcional do compilador.
   - *Próximo passo recomendado*: Inventário de visibilidade (Passo 1) e mapeamento dos 81 clusters para propor os nós modulares.
2. **Candidato #2 — `01_core/src/compiler/introspect.rs` (124 commits de corpo, 61 clusters, 12 fns de produção, 133 testes)**:
   - *Justificativa*: Maior arquivo absoluto da base. O fatiamento isolará os submódulos de extração de tags/labels/cites do `walk`, mantendo a suíte de 133 testes no hub per ADR-0104.
3. **Candidato #3 — `01_core/src/compiler/eval/rules.rs` (78 commits de corpo, 33 clusters, 30 fns de produção)**:
   - *Justificativa*: Separação lógica entre matching de show rules e engine de set rules.
4. **Candidato #4 — `01_core/src/compiler/layout/cursor.rs` (69 commits de corpo, 39 clusters, 27 fns de produção)**:
   - *Justificativa*: Módulo altamente acoplado de paginação e gestão de floats/footnotes.
5. **Candidato #5 — `01_core/src/compiler/math/layout/mod.rs` (56 commits de corpo, 27 clusters, 42 fns de produção)**:
   - *Justificativa*: Motor de layout de expressões e tabelas matemáticas (sujeito à emenda de constantes geométricas).
6. **Candidato #6 — `01_core/src/compiler/stdlib/layout.rs` (47 commits de corpo, 31 clusters, 33 fns de produção)**:
   - *Justificativa*: Nativas de container e grade da stdlib.
7. **Candidato #7 — `01_core/src/compiler/stdlib/collections.rs` (28 commits de corpo, 19 clusters, 67 fns de produção)**:
   - *Justificativa*: Baixo acoplamento histórico relativo; funções temáticas ortogonais.

---

## 6. Critérios de Conclusão

- [x] Medição factual de co-mudança histórica (`cochange_metrics.py`) executada e documentada.
- [x] Lista curta reordenada pelo Critério 3 (co-mudança), não por tamanho bruto.
- [x] Diagnóstico estrutural específico de `introspect.rs` per ADR-0104 (distinção entre os 133 testes e as 12 fns de produção).
- [x] Precedente de `cursor.rs` esclarecido e desvinculado de colisões documentais do P1062.
- [x] **Zero código alterado** neste passo.
