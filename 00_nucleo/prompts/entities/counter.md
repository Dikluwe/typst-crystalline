# Prompt L0 — `entities/counter` — chave de counter documental
Hash do Código: 9f7a1c4f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/entities.md`
**ADRs**: ADR-0107 (paridade linguagem), ADR-0127 (gate de contrato público)

---

## Contexto

`Counter` é o valor de primeira classe devolvido por `counter(selector)`. A
chave tem de distinguir dois espaços de nome independentes:

- **String de utilizador** — `counter("meu-counter")`, manipulado só via
  `.update()`/`.step()`/`.get()`.
- **Selector de elemento locatable** — `counter(heading)`, alimentado
  automaticamente pelo walk de `Heading`/`Figure`/`Table`/`Footnote`.

No vanilla a distinção é explícita via `CounterKey::Str` vs
`CounterKey::Selector` (`typst-library/src/introspection/counter.rs:527`). O
cristalino fundiu os dois numa única `EcoString`, o que causa colisão:
`counter("heading")` devolve o valor do contador automático de headings em
vez de `(0,)`.

## Instrução

### Contrato público

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CounterKey {
    /// Contador manual identificado por string de utilizador.
    Str(EcoString),
    /// Contador automático de elemento locatable.
    Selector(Selector),
    /// Contador de página (vanilla `CounterKey::Page`).
    Page,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub key: CounterKey,
}
```

- `CounterKey::Page` é incluído agora para paridade vanilla; consumers
  actuais que indexavam por `"page"` passam a usar a variante.
- `Hash` é necessário para usar `CounterKey` como chave de `HashMap` no
  `CounterRegistry`.
- `Selector` já implementa `Clone`, `PartialEq` e `Hash` em
  `entities/selector.rs` (confirmar; se faltar, adicionar).

### Construção

- `counter("foo")` → `Counter { key: CounterKey::Str("foo".into()) }`.
- `counter(heading)` / `counter(<selector>)` → `Counter {
  key: CounterKey::Selector(Selector::Kind(ElementKind::Heading)) }`.
- `counter(page)` (quando suportado) → `Counter { key: CounterKey::Page }`.

### Representação

- `repr(counter)` e mensagens de erro usam a representação vanilla da chave
  (ver `CounterKey::repr` no vanilla: `"heading"` para selector de heading,
  `"foo"` para string com repr de string).

## Gatilhos de reabertura

- Novo tipo de chave de counter no vanilla.
- Alteração ao `Selector` usado como chave.

## Critérios de verificação

```
#let c = counter("heading"); c.step()
#context counter("heading").get()  → (1,)   // counter manual
#context counter(heading).get()     → (0,)   // automático ainda não avançou

= T1
#context counter("heading").get()  → (1,)   // inalterado pelo heading
#context counter(heading).get()     → (1,)   // avançou uma vez
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

No teste counter_key_str_e_selector_sao_distintos, as únicas chaves efetivamente construídas são CounterKey::Str("heading") e Selector::Kind(Heading). Não possuem payload mutável na identidade; manter o HashMap, os dois inserts, len=2 e os dois gets. Autorizar somente #[expect(clippy::mutable_key_type, reason="CounterKey deste teste usa apenas Str e Selector::Kind imutáveis; o HashMap verifica namespaces distintos")] neste teste. Isto não declara segura a chave geral Selector/Value nem valida Eq para NaN. Uso de outra variante ou alteração das chaves reabre a decisão e exige teste de estabilidade de hash/igualdade antes de aceitar.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
