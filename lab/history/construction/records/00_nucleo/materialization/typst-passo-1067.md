# L0 — Passo 1067: Anotação Formal dos 11 Casos `2.0 × margin` / `2.0 × padding`

**Gate**: `ADR-0127` — mudança documental em massa (11 arquivos de código de
produção), mesma classificação já homologada no P1065. **Requer confirmação do
dono antes de executar**, mesmo mecanismo de cautela.

**Base**: P1066 (auditoria, 11/11 casos de produção confirmados — 10 por prova
estrutural do próprio tipo `PageConfig::margin: f64`, 1 por citação literal do
vanilla). Testes (`tests.rs`, 4 avisos) ficam fora do escopo — não são código de
produção.

---

## 1. Duas fundamentações distintas — não misturar na mesma anotação

Os 11 casos não têm a mesma prova, e a anotação deve reflectir isso, não usar um
texto genérico para os dois grupos.

### 1.1 Os 10 casos de margem (`columns.rs:146,161`, `footnote_flush.rs:60,62`,
`grid.rs:472,619`, `mod.rs:767,777,819,827`)

Prova **estrutural**, não de paridade externa: `PageConfig.margin` é `f64`
(escalar único, confirmado por citação directa de
`01_core/src/entities/layout_types.rs:570-580`) — logo
`left = right = top = bottom = margin` por definição do próprio tipo, e
`2.0 × margin` é verdade algébrica necessária, não uma escolha de design a citar
no vanilla.

Texto de anotação proposto (adaptar ao formato exacto já validado no P1065 para
`// rationale:`):

```rust
// rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom
// por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade
// algébrica estrutural, não paridade com o vanilla. P1066.
```

### 1.2 O caso de padding (`math/layout/frac.rs:53`)

Prova de **paridade literal com o vanilla** — `crates/typst-layout/src/math/
fraction.rs:56,104`, `let width = line_width + 2.0 * item.padding.at(size);`,
coincidência 1:1.

```rust
// rationale: padding simétrico dos dois lados da barra de fração — paridade
// literal com o vanilla (fraction.rs:56,104). P1066.
```

**Não usar o texto de §1.1 aqui** — a fundamentação é diferente (paridade
externa, não invariante de tipo), mesmo que a operação (`2.0 × algo`) seja igual
em forma.

## 2. Execução

- 11 arquivos/pontos anotados: 10 com o texto de §1.1, 1 com o texto de §1.2.
- Não tocar nos 4 avisos de `tests.rs` — fora de escopo (fixtures de teste, não
  código de produção; a pendência original nunca incluiu testes).
- Não reescrever nenhuma fórmula — só comentário. Zero mudança de comportamento.

## 3. Verificação

- `crystalline-lint --checks v21 .` — confirmar que os 11 avisos de produção
  desaparecem (4 de `tests.rs` continuam a aparecer, esperado — fora de escopo).
- `cargo test --workspace` — 100% pass.
- `crystalline-lint .` completo — 0 erros.

## 4. Critério de conclusão

- 11 anotações aplicadas, cada uma com a fundamentação correcta (§1.1 vs §1.2),
  não um texto genérico repetido nos dois grupos.
- Contagem de avisos V21 de produção antes/depois, explícita no relatório (mesmo
  padrão que faltou no P1065 e precisou de correcção).
- Os 4 avisos de `tests.rs` continuam presentes e são mencionados como
  intencionalmente fora de escopo, não esquecidos silenciosamente.

---

## Nota — pendências desta linha de trabalho, após este passo

Com P1067 concluído, a Categoria 1 original ("`/2.0`, `2.0 × margin`") fica
integralmente resolvida: 46 casos de `/2.0` (P1065) + 11 casos de `2.0 × margin`/
`padding` (este passo) = 57 anotações, todas com prova real, nenhuma por
suposição de simetria não verificada.

A segunda frente de "hardcode" desta conversa — expansão dos módulos de
constantes por domínio (`export/`, `stdlib/text/`, seguindo o piloto do P1058) —
continua disponível, não tocada por este passo.
