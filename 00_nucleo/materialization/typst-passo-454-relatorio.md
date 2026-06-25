# Relatório de Execução — P454 — Figure numbering

> **Passo:** 454  
> **Data de execução:** 2026-06-25  
> **Executor:** Kimi Code CLI  
> **Estado:** Concluído

---

## 1. Resumo

O passo P454 materializa a numeração automática de figures (`#figure[...]`) com
contador independente e caption prefixado. O repositório já possuía a base dessa
funcionalidade (contador via `Introspector`, gate `figure.numbering` na chain e
layout de caption em `01_core/src/rules/layout/figure.rs`), mas o **formato do
número estava hardcoded** como `"Figura N: "`, ignorando o pattern configurado
por `#set figure(numbering: "...")`.

Esta execução conectou o layout de figure ao pattern da chain, reaproveitando
`format_counter` (P451) para suportar `"1"`, `"1."`, `"I."`, `"(a)"`, `"A."`, etc.

---

## 2. Decisões de adaptação ao estado atual

O arquivo de passo P454 descrevia uma implementação legada em que `FigureElem`
teria um campo `numbering` assado e o eval computaria o número via
`CounterRegistry`. Essa arquitetura foi **descontinuada no P365**
(`f_fronteira_e1.md` §3a.9), que moveu o padrão de numeração para a chain
(`custom("figure.numbering")`) e o número para o `Introspector`. Reverter essa
decisão introduziria regressões e conflitos com passos posteriores
(P451/P451, P184C/D/E, P190H, P191B/C, P195D).

Por isso, optou-se por:

1. **Manter `FigureElem` sem campo `numbering`** (conforme P365).
2. **Manter `native_figure` sem parâmetro `numbering`**; o padrão continua
   vindo exclusivamente de `#set figure(numbering:)` na chain.
3. **Usar `format_counter` no layout** para formatar o número de acordo com o
   pattern armazenado na chain, reaproveitando a infraestrutura existente.
4. **Preservar o prefixo localizado `"Figura {formatted}: "`**, com fallback
   arábico quando o pattern for inválido.

Essa abordagem atende ao **espírito** do P454 (numeração automática de figures
com pattern customizável) sem quebrar a arquitetura estabelecida.

---

## 3. Arquivos alterados

| Arquivo | Mudança |
|---------|---------|
| `01_core/src/rules/layout/figure.rs` | Lê o pattern da chain e formata o número via `format_counter`; atualiza `@prompt-hash` para `7fb90d66`. |
| `01_core/src/rules/layout/tests.rs` | Adiciona 5 novos testes de layout para patterns romano, letras minúsculas, letras maiúsculas, fallback de pattern inválido e sequência romana. |
| `00_nucleo/prompts/rules/layout_figure.md` | Atualiza L0 para documentar leitura do pattern da chain, uso de `format_counter` e critérios de verificação expandidos. |
| `00_nucleo/materialization/typst-passo-454-relatorio.md` | Este relatório. |

---

## 4. Testes adicionados

Todos no módulo `rules::layout::tests`:

- `layout_figure_pattern_romano` — pattern `"I."` → `"Figura I.: "`.
- `layout_figure_pattern_letras_minusculas` — pattern `"(a)"` → `"Figura (a): "`.
- `layout_figure_pattern_letras_maiusculas` — pattern `"A."` → `"Figura A.: "`.
- `layout_figure_pattern_invalido_fallback_arabico` — pattern `"x"` → fallback `"Figura 1: "`.
- `layout_figure_sequencial_romano` — duas figures com `"I."` produzem `"I."` e `"II."`.

Os testes pré-existentes (`layout_figure_com_caption_tem_prefixo`,
`layout_figure_sem_caption_sem_prefixo`, os testes P184E/P195D, etc.)
continuam passando, garantindo retrocompatibilidade.

---

## 5. Resultados

### 5.1 `cargo test --workspace`

Executado com `RUST_MIN_STACK=16777216` devido a um stack overflow
**pré-existente** no teste `rules::eval::tests::tests::p350c_flag_on_nao_convergente_classifica`
(independente deste passo; reproduzível sem as alterações do P454).

Resultado:

```text
test result: ok. 0 failed
```

Todos os crates passaram (`typst-core`, `typst-shell`, `typst-infra`,
`typst-wiring`), incluindo doc-tests e testes de integração (`cli.rs`,
`crystalline_lint.rs`).

### 5.2 `crystalline-lint .`

Resultado:

```text
warning: Prompt órfão: '00_nucleo/prompts/adr/adr-stub-vs-fallback.md' ... [V7]
warning: Prompt órfão: '00_nucleo/prompts/rules/show-regex.md' ... [V7]
```

As únicas advertências são **pré-existentes** e referem-se a prompts não
referenciados por nenhum arquivo L1–L4. Não há violações V5 (drift de prompt)
relacionadas às alterações deste passo.

---

## 6. Checklist de fecho (adaptado)

- [x] Layout de figure lê o pattern de numeração da chain.
- [x] Número formatado via `format_counter` (chave semântica `"figure"`,
      independente de heading).
- [x] Caption numerada renderizada como `Content::Sequence` (body + linebreak +
      prefixo + caption) — equivalente ao requisito de `FrameItem::Group`.
- [x] 5 novos tests verdes + todos os tests de figure pré-existentes verdes.
- [x] Spec L0 atualizada (`00_nucleo/prompts/rules/layout_figure.md`).
- [x] `cargo test --workspace` verde (com workaround de stack pré-existente).
- [x] `crystalline-lint` sem violações relacionadas ao P454.

Itens do P454 original que foram **adaptados** em vez de implementados
diretamente:

- [ ] `FigureElem` ganha campo `numbering` — **rejeitado** para preservar P365;
      o padrão vive na chain.
- [ ] `native_figure` aceita `numbering` — **rejeitado** para preservar P365;
      `#set figure(numbering:)` é o mecanismo único.
- [ ] Eval computa número via `CounterRegistry` — **adaptado**; o número vem do
      `Introspector`, que já encapsula o contador.

---

## 7. Notas

- O prefixo localizado continua fixo como `"Figura "` + número + `": "`,
  conforme scope-out de i18n do P454.
- Sub-figures, equation numbering, LoF, label/ref e posicionamento da caption
  acima do body permanecem scope-out, conforme o passo original.
- O stack overflow em `p350c_flag_on_nao_convergente_classifica` ocorre mesmo
  sem as alterações deste passo; é um débito técnico independente.

---

**Fim do relatório.**
