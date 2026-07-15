---
# P766 — Verificação de completude: tabelas de símbolos além de `arrow`

> **Passo:** 766
> **Data:** 2026-07-15
> **Foco:** P765a implementou `symbol()` e modificadores via field access, mas a tabela de variantes de `arrow` cobre só um subconjunto medido (52 variantes) contra a tabela completa do vanilla — divergência justificada sob ADR-0107 como "expansão de dados, não mudança de linguagem". Essa justificação foi verificada **só para `arrow`**. Não há confirmação de que os outros grupos de símbolos (`sym.gt`, `sym.eq`, `sym.dot`, `sym.tilde`, categorias matemáticas, emoji, etc.) tenham a mesma cobertura relativa, ou se algum tem lacunas maiores que passariam despercebidas pelo mesmo raciocínio.
> **Tipo:** Sonda de verificação + Implementação (expansão de tabela) onde a lacuna for desproporcional.
> **Tamanho:** M — depende de quantos grupos existem e de quão desiguais forem as coberturas.
> **ADR-0107 EM VIGOR** — critério de divergência aceitável (dados vs língua) só se aplica quando o mecanismo de aceitação/rejeição de modificadores está correcto; uma tabela vazia ou quase vazia para um grupo inteiro não é "expansão de dados", é ausência funcional.
> **ADR-0108 EM VIGOR** — medir cada grupo, não extrapolar de `arrow` para os restantes.
> **Dependências:** P765a (constructor/modifiers de `Symbol` implementados; `sym.rs` como ponto de partida).

---

## Sonda

### 1. Levantar todos os grupos de símbolos no vanilla

```bash
grep -n "^pub const\|Symbol::new\|symbols! {" \
  lab/typst-original/crates/typst-library/src/foundations/symbols.rs 2>/dev/null | head -50
grep -c "=>" lab/typst-original/crates/typst-library/src/foundations/symbols.rs 2>/dev/null
```

Confirmar o mecanismo real de definição de símbolos no vanilla (arquivo gerado, macro, tabela estática) e contar quantas entradas/variantes cada grupo top-level tem (`arrow`, `gt`, `eq`, `dot`, `tilde`, categorias gregas, matemáticas, emoji, etc.).

### 2. Levantar a cobertura actual no cristalino

```bash
grep -n "\"arrow\"\|\"gt\"\|\"eq\"\|\"dot\"\|\"tilde\"" 01_core/src/rules/stdlib/sym.rs
wc -l 01_core/src/rules/stdlib/sym.rs
```

Construir uma tabela: grupo → nº de variantes no vanilla → nº de variantes no cristalino → % de cobertura.

### 3. Priorizar por uso real, não por contagem bruta

Para grupos com cobertura muito baixa (ex: <20%), verificar se algum consumidor conhecido do projecto (`cetz`, os documentos de teste do corpus, `estado-geral-p695.md`) usa alguma das variantes ausentes. Não expandir tabelas inteiras sem uso identificado — regra de não implementar sem sonda de uso real (ADR-0114, espírito).

```bash
grep -rn "sym\.\w\+\.\w\+" lab/typst-original/tests/ 00_nucleo/testing/ 2>/dev/null | head -30
```

---

## Implementação (condicional ao resultado da sonda)

- Para grupos com lacuna real e uso identificado: expandir a tabela de variantes seguindo o mesmo padrão já estabelecido em `sym.rs` por P765a (não redesenhar o mecanismo).
- Para grupos com lacuna real mas sem uso identificado no corpus actual: registar em `achados-adiados-cetz.md` ou ficheiro equivalente como scope-out consciente, não implementar especulativamente.
- Corrigir `repr()` se a expansão revelar formatação incorrecta para algum caso não coberto por P765a.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para cada grupo expandido, repetir o padrão de comparação de P765a (`repr()` vanilla vs cristalino lado a lado).

---

## Critério de fecho do passo

- [ ] Tabela de cobertura por grupo de símbolo construída (vanilla vs cristalino, contagem real).
- [ ] Grupos com lacuna desproporcional identificados e cruzados com uso real no corpus.
- [ ] Grupos expandidos por uso confirmado, testados com `repr()` comparado ao vanilla.
- [ ] Grupos sem uso identificado registados como scope-out consciente, não implementados especulativamente.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p766.md`, com a tabela de cobertura completa.

---

## Próximo passo

Nenhum passo automático — decisão de dono sobre se a cobertura restante (grupos sem uso identificado) entra em algum lote futuro de P765 ou fica scope-out permanente.
