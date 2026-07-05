# P576 — Alinhamento de parágrafo com `#set text(dir: rtl)`

**Status**: `SONDA CONCLUÍDA` — implementação bloqueada pela Trava Arquitetural; aguarda confirmação humana dos Prompts L0 actualizados.  
**Data**: 2026-07-05  
**Scope**: `01_core/src/rules/eval/rules.rs`, `01_core/src/rules/eval/mod.rs`, `01_core/src/entities/value.rs`, `01_core/src/entities/dir.rs`, `01_core/src/rules/stdlib/text.rs`, `01_core/src/rules/layout/mod.rs`, `01_core/src/rules/layout/text.rs`, `03_infra/src/layout_bidi.rs`.  
**Commit da medição**: `7764dbb44`.

---

## 1. Pergunta

A sequência RTL está quase completa: ordem das palavras dentro da linha (P562/P564/P567) e espaços entre palavras (P569) já funcionam. Falta o alinhamento de parágrafo: um texto árabe começa sempre na margem esquerda, mesmo quando deveria começar à direita com `#set text(dir: rtl)`. Este passo sonda onde a mudança deve ser feita e que Prompts L0 precisam de actualização.

---

## 2. Medições

### 2.1 `dir:` é reconhecido como propriedade, mas `rtl` não existe no escopo

`01_core/src/rules/eval/rules.rs:931` — o arm `target == "text"` trata `bold`, `italic`, `size`, `fill`, `weight`, `tracking`, `lang`, `font`; qualquer outra chave cai no warn de propriedade não suportada (`unsupported_property_warn`, linha 1077).

Teste:

```bash
./target/release/typst /tmp/p576-dir.typ /tmp/p576-dir.pdf
# /tmp/p576-dir.typ: #set text(dir: rtl) + مرحبا بالعالم
```

Resultado:

```text
/tmp/p576-dir.typ:1:16: error: unknown variable: rtl
```

Com string `"rtl"`:

```text
/tmp/p576-dir-str.typ:1:11: warning: text: propriedade 'dir' ainda não suportada
```

Conclusão: o parser aceita `dir:`; o eval não reconhece o identificador `rtl` nem trata a chave `dir`.

### 2.2 Identificadores de alinhamento já seguem o mesmo padrão

`01_core/src/rules/eval/mod.rs:1196-1203` — `left`, `center`, `right`, `start`, `end`, `top`, `horizon`, `bottom` são definidos como `Value::Align` no escopo global. O equivalente para direcções (`ltr`, `rtl`, `ttb`, `btt`) ainda não existe.

### 2.3 O alinhamento inicial do parágrafo é fixo na margem esquerda

`01_core/src/rules/layout/mod.rs:499-501` — `cursor_x` e `line_start_x` inicializam-se em `margin`;
não há ramificação por direcção do texto.

### 2.4 `rules/layout/text.rs` não lê `text.dir`

`01_core/src/rules/layout/text.rs:31-60` — decodifica `text.size`, `text.fill`, `text.weight`, `text.tracking`, `text.lang`, `text.font`; `text.dir` não está na lista.

### 2.5 A detecção automática de RTL não alinha o parágrafo

Teste sem `dir:`:

| Ferramenta | Input | Resultado visual |
|------------|-------|------------------|
| Cristalino | `مرحبا بالعالم` | Texto começa à **esquerda**. |
| Vanilla    | `مرحبا بالعالم` | Texto começa à **esquerda**. |

Teste com `dir: rtl`:

| Ferramenta | Input | Resultado visual |
|------------|-------|------------------|
| Vanilla    | `#set text(dir: rtl)`<br>`مرحبا بالعالم` | Texto começa à **direita**. |
| Cristalino | `#set text(dir: rtl)`<br>`مرحبا بالعالم` | Erro (`unknown variable: rtl`). |

A detecção automática (`unicode-bidi`) decide a ordem das palavras dentro da linha, mas não decide a margem inicial do parágrafo. O vanilla exige `dir: rtl` explícito para alinhar à direita.

---

## 3. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| Implementar agora sem actualizar L0? | **Não** | A Trava Arquitetural exige L0 actualizado antes de código L1/L2/L3. O L0 `infra/layout_bidi.md` scope-out explicitamente o alinhamento de parágrafo. |
| Adicionar `Value::Dir(Dir)`? | **Sim** | Para suportar `dir: rtl` como identificador (paridade sintática) é preciso um valor de runtime; `Dir` já existe em L1. |
| Onde controlar o alinhamento? | **Layouter (L1)** lê `text.dir` e ajusta a origem do parágrafo; `layout_bidi` (L3) continua a reordenar palavras. | `layout/mod.rs:499-501` é o ponto onde a origem é fixada. Fazer só em L3 esconderia o problema da quebra de linha. |
| `ltr`/`rtl`/`ttb`/`btt` no escopo global? | **Sim** | Mesmo padrão de `left`/`center`/`right` (`eval/mod.rs:1196-1203`). |
| Scope-out deste passo? | **ttb/btt** — escrita vertical fica para passo dedicado, conforme scope-out já registado. | `infra/layout_bidi.md` mantém scope-out de scripts top-down. |

---

## 4. Prompts L0 actualizados

Foram actualizados os Prompts L0 necessários para legitimar a implementação:

- `00_nucleo/prompts/entities/value.md` — adiciona `Value::Dir(Dir)`.
- `00_nucleo/prompts/entities/dir.md` — inclui `Value::Dir` e `text.dir` como consumers.
- `00_nucleo/prompts/rules/stdlib/text.md` — adiciona a propriedade `dir` em `#set text(...)`.
- `00_nucleo/prompts/infra/layout_bidi.md` — remove o scope-out de alinhamento de parágrafo e especifica o comportamento com `text.dir`.

Os headers `@prompt-hash` nos ficheiros L1 foram sincronizados com `crystalline-lint --fix-hashes .`.

---

## 5. Conclusão

A sonda está concluída. A implementação de `#set text(dir: rtl)` requer:

1. Adicionar `Value::Dir(Dir)` em `entities/value.rs`.
2. Expor `ltr`/`rtl`/`ttb`/`btt` no escopo global do eval.
3. Reconhecer `dir` em `eval_set_rule` target `text`.
4. Ler `text.dir` em `rules/layout/text.rs` e transportá-lo no `TextStyle`.
5. Ajustar a origem horizontal do parágrafo no Layouter quando `dir == RTL`.
6. Validar com documentos árabe/latino mistos e confirmar ausência de regressão em P569.

Por força do Protocolo de Nucleação, **a implementação não prossegue até confirmação humana** de que os Prompts L0 actualizados foram guardados e os seus hashes verificados.
