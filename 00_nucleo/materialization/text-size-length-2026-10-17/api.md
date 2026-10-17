# Contrato público proposto TS1

```rust
// antes
Style::Size(Pt)
StyleDelta { pub size: Option<f64>, /* demais campos */ }
// depois
Style::Size(Length)
StyleDelta { pub size: Option<Length>, /* mesmos demais campos */ }
```

Length já existe e preserva abs + em. Nenhum tipo/trait/campo extra, alias paralelo ou nova dependência é proposto. TextStyle.size continua Pt; StyleChain::size continua retornando f64 resolvido; Length e native_text mantêm assinaturas.

Migração Rust: `Style::Size(Pt(v))` → `Style::Size(Length::pt(v))`; `size: Some(v)` → `size: Some(Length::pt(v))`. Valores já resolvidos não são reinterpretados como relativos. Literais externos, pattern matches e chamadas que esperam f64 em delta.size precisarão migrar. Essa quebra exige ADR-0127 §2.1/§2.4.

Fórmula declarativa: interno ai+ei*x sobre externo ao+eo*x → (ai+ei*ao)+(ei*eo)*x. Collapse de prefixo sem ancestral conserva em; não insere 11pt antecipadamente. Default final permanece11pt. Não há deslocamento eval↔layout nem novo modo/flag.

Memória atual medida em sizes.json: Length16, Option<Length>24, Option<f64>16, Style32 e StyleDelta/Styles328bytes. O payload do campo cresce8bytes; estimativa do delta336bytes sob o alinhamento atual, ainda não medida em código materializado. Style já tem variantes Length. Medir layout/alocações e pico após implementação; hard12GiB/swap0 continua obrigatório. Não afirmar melhoria de RAM.
