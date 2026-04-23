# Auditoria global de DEBTs — Passo 105

Data: 2026-04-23.

Total de DEBTs no ficheiro: **53**.
- Secção 1 (abertos / parcialmente resolvidos): **14**.
- Secção 2 (encerrados / resolvidos): **39**.

---

## 105.A — Inventário completo dos DEBTs em aberto

| DEBT | Título | Estado actual | Critério descrito? | Último passo | Classif. |
|------|--------|--------------|--------------------|--------------|---------:|
| DEBT-1 | StyleChain — pendências residuais | PARCIALMENTE RESOLVIDO (estrutura paga em Passo 100) | Sim, multi-partes | 104 (Actualização Passo 104) | **M** |
| DEBT-2 | Closures eager vs lazy capture | PARCIALMENTE RESOLVIDO | Sim (comemo + paridade avançada) | 31 | **M** |
| DEBT-8 | Motor de equações | PARCIALMENTE RESOLVIDO | Sim (kern, fontes MATH, MathPrimes, baseline) | 84.1 | **M** |
| DEBT-9 | Cobertura de paridade — tracking contínuo | Baseline estabelecido (35); tracking ongoing | Por design contínuo | 35 | **M** |
| DEBT-33 | Bounding Box de curvas Bézier | EM ABERTO | Sim (cálculo analítico extremos) | 79 | **M** |
| DEBT-34d | Auto não encolhe antes de matar fr | EM ABERTO | Sim (min-content/max-content) | 80 | **M** |
| DEBT-34e | colspan e rowspan | EM ABERTO | Sim (algoritmo placement) | 80 | **M** |
| DEBT-35b | Invalidação de cache available_width após SetPage | EM ABERTO (preventivo) | Sim (accionável se cache aparecer) | 81 | **M** |
| DEBT-42 | `get_unchecked` no scanner | EM ABERTO, bloqueado por benchmark | Sim (ADR-0032, depende de infra benchmark) | 84.8a | **M** |
| DEBT-43 | Linter whitelist crate-level vs type-level | EM ABERTO | Sim (4 checkboxes) | 89 | **M** |
| DEBT-45 | Métodos `check_*_depth` não chamados pelo eval | EM ABERTO (parcialmente pago Passo 93) | Sim (4 checkboxes; 2/4 ✓) | 93 | **M** |
| DEBT-49 | Propriedades de `#set` silenciadas | EM ABERTO | Sim (4 checkboxes; 1/4 ✓) | 104 (Actualização) | **M** |
| DEBT-50 | Show selector origin (dívida latente) | EM ABERTO com canário | Sim (activa-se se `#set text` migrar para wrapping) | 103 | **M** |
| DEBT-51 | Warnings do `Sink` → L3/CLI | EM ABERTO | Sim (decidir canal; implementar) | 104 | **M** |

---

## 105.B — Classificação e evidência

### Conclusão da auditoria

**Todos os 14 DEBTs activos classificados como M (Manter)**. Nenhum
qualifica para F (Fechar) ou A (Actualizar).

### Evidência por DEBT

#### DEBT-1 (M)

- Header: "PARCIALMENTE RESOLVIDO (estrutura paga em Passo 100)".
- Já tem secções de "Actualização Passo 99", "100", "101", "102", "103",
  "104" adicionadas em cada passo. Estado actual do texto reflecte
  exactamente o código.
- Pendências restantes (propriedades adicionais bloqueadas por tipos
  não materializados) estão correctamente descritas.
- Sem acção. Forma actual é comprehensive mas proporcional à
  extensão real da dívida.

#### DEBT-2 (M)

- Pendências: "Integração com comemo para tracking semântico real"
  + "Testes de paridade avançados".
- Nada mudou desde Passo 31. `Arc<Scope>` snapshot eager continua a
  ser a implementação. Comemo tracking não foi activado.
- Texto está correcto.

#### DEBT-8 (M)

- Pendências: "Kern matemático", "Fontes OpenType MATH",
  "MathPrimes", "Baseline correcta x-height".
- Passos 41–50 trataram de parte do motor (OpenType MATH parcial,
  layout de matrix/cases). Passo 96.8 reestruturou `math/layout.rs`
  mas não adicionou features novas ao motor em si.
- Texto tem nota "Actualização no Passo 84.1" removendo `MathAlignPoint`
  (já implementado). Restantes continuam válidas.

#### DEBT-9 (M)

- Por design contínuo — sem critério de fecho. Baseline 35 preservado.
- Sem mudança empírica desde então (parity runner passa 50/50).

#### DEBT-33 (M)

- `ShapeKind::Path` ainda usa min/max dos pontos de controlo (grep em
  `01_core/src/rules/layout/` confirma ausência de cálculo analítico
  Bezier).
- Texto correcto.

#### DEBT-34d (M)

- Auto greedy continua — grep em `rules/layout/grid.rs` mostra
  `let safe = (available_width - total_fixed_w).max(0.0);` passado a
  Auto measure. Sem negociação min-content/max-content.
- Texto correcto.

#### DEBT-34e (M)

- `Content::Grid { columns, rows, cells }` não tem colspan/rowspan
  nos args; layout em `grid.rs` usa `cells.chunks(num_cols)` — linear.
- Sem progresso.

#### DEBT-35b (M)

- `available_width()` em `layout/mod.rs:152` permanece cálculo inline,
  sem cache. Comentário `// DEBT-35b:` em `layout/mod.rs:507` é
  preventivo.
- Se alguém introduzir cache no futuro, o comentário orienta a
  adicionar invalidação no arm `Content::SetPage`. DEBT fica
  documentando o risco.

#### DEBT-42 (M)

- `grep -c "get_unchecked" 01_core/src/rules/lexer/scanner.rs` = 7
  ocorrências (confirmado).
- Bloqueio (infra benchmark) permanece.

#### DEBT-43 (M)

- `crystalline.toml` continua com whitelist crate-level (não
  type-level). Alteração depende do projecto externo
  `crystalline-lint`.
- Sem progresso neste repo.

#### DEBT-45 (M — parcialmente pago, estado correctamente descrito)

- Secção "Estado actual (após Passo 93)" detalha 4 checkboxes:
  `check_call_depth` ✓, `check_show_depth` ✓,
  `check_layout_depth` ✗ (adiado para Engine<'a>),
  `check_html_depth` ✗ (aguarda pipeline HTML).
- 2/4 pendentes — não qualifica para F.

#### DEBT-49 (M)

- Recém-aberto (Passo 102), actualizado Passo 104.
- 1/4 checkboxes pago (Sink materializado).
- Propagação + migração de consumer silenciados continuam pendentes.

#### DEBT-50 (M)

- Recém-aberto (Passo 103) com teste-canário em
  `layout/tests.rs::tests_show_rule_integration::debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`.
- Canário passa → dívida adormecida. Activa-se se/quando `#set text`
  migrar para wrapping.

#### DEBT-51 (M)

- Recém-aberto (Passo 104). Estado correctamente descrito.
- Critério: decidir canal (tuple ou TrackedMut caller-managed) +
  implementar + testar integração L1→L3.

---

## Verificação adicional: DEBTs fechados, orphan comments

### DEBTs fechados recentes em Secção 2

Confirmados presentes na Secção 2:

- DEBT-46 (96.10), DEBT-47 (97), DEBT-48 (100) — todos correctos.
- DEBT-44 (92), DEBT-40 (90), DEBT-41 (85), DEBT-39 (95) — todos
  correctos.

### Orphan comments

Grep por `// DEBT-`/`/* DEBT-` em `01_core/src/` e `03_infra/src/`:

```
entities/counter_state.rs:27  /// DEBT-10: Resolver contadores em duas passagens [...] quando o motor de introspecção...
entities/content.rs:171       /// DEBT-10: substituir por StyleChain quando o motor de introspecção...
entities/content.rs:178       /// DEBT-10: single-pass não suporta referências para a frente.
entities/content.rs:255       rows: Vec<TrackSizing>, // DEBT-34b: ignorado — todas as linhas são Auto
rules/layout/mod.rs:507       // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
rules/layout/mod.rs:531       // DEBT-28 encerrado: intrinsic_width/height vêm de calculate_dimensions.
rules/eval/tests.rs:1964      // DEBT-19 encerrado: heading dentro de sequence deve ser intercetado.
rules/eval/tests.rs:2007      // DEBT-20 encerrado: a regra transforma heading em heading.
rules/eval/tests.rs:2051      // DEBT-23: com múltiplas regras NodeKind, map_content é chamado uma vez.
```

Classificação (spec "se diz 'encerrado' ou é nota histórica, deixar;
se diz 'pendente', remover"):

- `DEBT-10` em content.rs e counter_state.rs: falam de "substituir
  quando motor de introspecção". Contexto: DEBT-10 está resolvido
  (contadores em duas passagens), mas estes comentários referem-se
  a **trabalho futuro condicionado** à materialização de `Introspection`.
  Mantêm-se como forward-looking notes. **Deixar.**
- `DEBT-34b` em content.rs:255: comentário documenta ignorância
  actual do parâmetro `rows`. DEBT-34b está encerrado (Passo 83).
  Agora `rows` **é** usado no layout. Verificar se o comentário
  está obsoleto.
  
  Vou verificar em `content.rs:255` se o comentário reflecte o
  código actual ou não. Se `rows` é usado hoje, o comentário é
  obsoleto. Senão, é nota histórica.

Actually let me check. Let me do the check now.
