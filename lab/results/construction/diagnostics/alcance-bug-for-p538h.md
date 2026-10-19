# Alcance histórico do bug de `#for` — P538h

**Data:** 2026-07-03  
**Passo:** 538h  
**Tipo:** Diagnóstico sem código de produção  
**Dependências:** P538f (correcção do bug), P514 (paridade de linguagem completa)

## Perguntas a responder

1. O corpus de P490–P514 tinha algum documento com `#for` a produzir texto/conteúdo visível?
2. Se tinha, o teste verificava o conteúdo ou só que compilava?
3. Há quanto tempo o descarte de conteúdo existe?
4. O corpus histórico continua a compilar após o fix de P538f?

## Grupo 1 — Procura de `#for` no corpus histórico

Comando executado:

```bash
grep -rln "#for" lab/parity/corpus/
```

Resultado: **nenhum ficheiro encontrado**.

Foi também procurado `"for "` e `"for("` como variações possíveis:

```bash
grep -rln "for " lab/parity/corpus/
grep -rln "for(" lab/parity/corpus/
```

Resultado: apenas ocorrências em metadados `.toml` (`diagnostic_for = ...`),
não código Typst.

Conclusão: **não existem documentos Tipo A nem Tipo B** com `#for` no corpus
histórico. O bug não podia ter sido detectado por este corpus, simplesmente
porque a funcionalidade não era exercitada.

## Grupo 2 — Histórico de `control_flow.rs`

Comando executado:

```bash
git log --oneline -- 01_core/src/engine/eval/control_flow.rs | tail -40
```

Resultado relevante:

```
2ff56ebad P538f: #for acumula conteúdo do corpo
2ffc40dca P536 — Metadados do documento (/Info)
a63e739d2 P498: separacao conteudo original vs output show-rule
1a1151b7a P497: validacao formal dos gaps D4/D5
...
08de07dba Passo 96
```

Foi inspeccionado o estado do ficheiro no commit de P514:

```bash
git show 94ed17dba:01_core/src/engine/eval/control_flow.rs | sed -n '50,90p'
```

Nesse commit, `eval_for` já continha:

```rust
for item in items {
    ctx.tick_loop(loop_expr.span())?;
    scopes.enter();
    scopes.define(name.as_str(), item);
    eval_expr(loop_expr.body(), scopes, ctx, engine)?;  // resultado descartado
    scopes.exit();
}
Ok(Value::None)
```

Foi também inspeccionado o commit inicial do ficheiro (P96):

```bash
git show 08de07dba:01_core/src/engine/eval/control_flow.rs
```

A mesma estrutura de descarte já existia.

Conclusão: **o bug é histórico desde a introdução do `eval_for`**, não foi
introduzido depois de P514. P514 não o invalida como quebra recente.

## Grupo 3 — O que a bateria P490–P514 de facto verificava

Foram analisados três ficheiros de teste em `lab/parity/tests/`:

### `structural_parity.rs`

- Compara `typst query` entre cristalino e vanilla.
- Selectors: `heading`, `figure`, `metadata`, `math.equation`.
- **Não extrai texto** nem verifica conteúdo textual.
- Um `#for` que gera texto simples (sem headings, figuras, etc.) não apareceria
  como diferença estrutural.

### `layout_parity.rs`

- Cristalino-only baseline.
- Compila o corpus e extrai `FrameDTO`.
- **Sem `assert!` global** — paridade é medição, não verificação.
- Não compara conteúdo textual com vanilla.

### `eval_parity.rs`

- Avalia corpus `semantic/` e compara `__resultado__`.
- O corpus `semantic/` não contém `#for`.

Conclusão: **a bateria P490–P514 era estrutural/quantitativa, não de conteúdo
textual**. Um `#for` que perde o conteúdo do corpo não seria detectado: o
documento continuaria a compilar e a estrutura de elementos query-able
permaneceria inalterada (vazia, mas válida).

## Grupo 4 — Re-teste do corpus histórico com o fix de P538f

Comando executado:

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  ./target/release/typst "$f" /tmp/out-p538h.pdf >/dev/null 2>&1 && \
    echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Resultado: **todos os 39 ficheiros compilaram com sucesso**.

Não há regressões introduzidas pelo fix de P538f.

## Tabela final

| Pergunta | Resposta |
|----------|----------|
| Existia `#for` Tipo B no corpus P490–P514? | **Não.** Não há nenhum `#for` no corpus. |
| Se existia, a bateria verificava conteúdo ou só estrutura? | **Não aplicável**, mas a bateria era estrutural (query) sem verificação de texto. |
| Há quanto tempo o descarte de conteúdo existe? | **Desde o início** do `control_flow.rs` (Passo 96). Não é uma quebra recente. |
| Corpus histórico sem regressão pós-fix? | **Sim.** Todos os 39 ficheiros de p490/p500 compilam. |

## Decisão de prosseguimento

O bug de `#for` era um **ponto cego do método de teste**, não uma invalidação
da alegação de P514 sobre o que foi testado. O corpus não exercitava `#for` e
a bateria não verificava conteúdo textual. O fix de P538f não introduz
regressões no corpus histórico.

A reorganização pode prosseguir. Recomenda-se reforçar a bateria futura com
casos de `#for` em modo markup e verificação de conteúdo textual (ex.:
`pdftotext`, ou comparação de `FrameDTO.text_content`).
