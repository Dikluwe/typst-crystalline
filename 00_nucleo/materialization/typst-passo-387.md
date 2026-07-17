# Passo 387 — Materialização: data-loading (`read`/`csv`/`json`/`yaml`/`toml`/`cbor`/`xml`)

**Tipo**: Materialização (toca L1 + L3; sem variant `Content` novo — são funções stdlib que devolvem `Value`).
**Data**: 2026-06-21.
**Padrão**: diagnóstico-primeiro já cumprido (achado na Lista B do Passo 386); inventariar-primeiro (ADR-0065); medir-antes-de-decidir (ADR-0108).
**Achado fonte**: `typst-falta-migrar-lista-B-passo-386.md` §3 — cluster `loading` ausente em L1, não catalogado, e **bloqueante de `bibliography`** (a maior dívida Model, XL).
**ADRs relevantes**: ADR-0029 (pureza L1 / zero I/O), ADR-0033 (paridade vanilla), ADR-0107 (paridade é com a língua), ADR-0054 (perfil graded), e uma ADR nova de autorização de crates de parser (§ADR abaixo).

> **Nota de numeração de ADR.** Este passo autoriza crates externas de parsing → exige ADR. O número fica `ADR-NNNN`: varrer `00_nucleo/adr/` e usar o primeiro livre (ADR-0110 ocupada; 0111 provável). Precedente P160A/P376 — nunca assumir número.

---

## 1. Contexto

A Lista B do Passo 386 isolou o único produto líquido da rede bottom-up: o módulo `typst_library::loading` está ausente em L1 (grep confirmou zero hits em `rules/stdlib` para `csv`/`json`/`yaml`/`cbor`/`xml`), não é catalogado pelo Inventário 148 (só mencionado como bloqueante de `bibliography`), e é feature user-facing real.

Materializá-lo tem alavanca dupla: fecha uma lacuna confirmada **e** destrava o cluster mais pesado de Model (bibliography/cite, XL), que depende de carregar ficheiros de dados.

### Superfície vanilla (alvo)

| Função | Símbolo vanilla | Devolve |
|--------|------------------|---------|
| `read(path)` | `loading::read_::read` | `Value::Str` ou `Value::Bytes` |
| `csv(path, …)` | `loading::csv_::csv` (+ `Delimiter`, `RowType`) | `Value::Array` de linhas |
| `json(path)` | `loading::json_::json` | `Value` (árvore) |
| `yaml(path)` | `loading::yaml_::yaml` | `Value` (árvore) |
| `toml(path)` | `loading::toml_::toml` | `Value` (árvore) |
| `cbor(path)` | `loading::cbor_::cbor` | `Value` (árvore) |
| `xml(path)` | `loading::xml_::xml` (+ `convert_xml`) | `Value::Array` de nós |
| (infra) | `DataSource`, `LoadSource`, `Loaded`, `Readable`, `Encoding` | — |

---

## 2. Decisão de engenharia (a melhoria sobre o vanilla)

O vanilla acopla, em cada função, a leitura de disco ao parsing. O perfil de pureza L1 do cristalino (ADR-0029, zero I/O) **proíbe** esse acoplamento — e isso, em vez de obstáculo, é a engenharia melhor. A função parte em duas, por estrato:

- **L1 — decode puro.** `decode_csv(bytes, opts) -> Result<Value>`, `decode_json(bytes) -> Result<Value>`, etc. Entrada: bytes já em memória. Zero I/O. Determinístico. Testável sem disco. É aqui que vive a lógica de paridade com a língua (ADR-0107).
- **L3 — leitura.** `read_bytes(path) -> Result<Vec<u8>>`. O único sítio que toca disco. Adapter fino.
- **Fiação (L4 / stdlib registration).** A função stdlib user-facing `csv(path)` compõe: L3 lê bytes, L1 decodifica. A composição é a única que conhece os dois.

Consequência testável: o decode L1 é coberto por testes com bytes literais (sem fixture de disco); o adapter L3 por um teste de I/O isolado. A paridade de **língua** (formato de saída do `csv`/`json`/…) prova-se toda em L1, puro.

> Esta separação não é over-engineering imposto: ela cai do estrato. O ganho concreto é que o comportamento user-facing (o formato do `Value` devolvido) fica num módulo puro, re-executável e diffável contra o vanilla sem montar disco — exatamente o eixo de paridade que importa.

---

## 3. ADR de autorização de crates (PROPOSTO neste passo)

Parsing exige crates externas. Propor `ADR-NNNN` autorizando, com precedente (ADR-0024 ecow, ADR-0023 indexmap, ADR-0057 hypher, ADR-0062 hayagriva):

| Formato | Crate candidata | Nota |
|---------|-----------------|------|
| json | `serde_json` | já provável no grafo (serde) |
| yaml | `serde_yaml` (ou sucessor mantido) | confirmar manutenção; alternativa pura se houver |
| toml | `toml` | já provável (Cargo usa) |
| cbor | `ciborium` | puro-Rust, sem C |
| xml | `roxmltree` ou `quick-xml` | preferir parser sem alloc pesada |
| csv | parser próprio L1 ou `csv` | **avaliar parser próprio**: CSV é simples; um decode L1 sem crate mantém L1 mais puro e evita dep |

Critério de escolha por crate, na ADR: pureza (no_std/sem I/O próprio se possível), manutenção viva, ausência de C/build-script frágil. **Decisão por formato, documentada** — não autorização em bloco.

---

## 4. O que produzir

1. **L1 decode puro** em `01_core/src/engine/stdlib/` (módulo `loading` novo): `decode_{csv,json,yaml,toml,cbor,xml}(bytes, opts) -> Result<Value, …>` + `Encoding`/`Readable` conforme necessário. Sem I/O.
2. **L3 leitura** em infra: `read_bytes(path) -> Result<Vec<u8>>` (adapter de disco; o único sítio I/O).
3. **Stdlib funcs** registadas em `make_stdlib`: `read`, `csv`, `json`, `yaml`, `toml`, `cbor`, `xml` — cada uma compõe L3+L1.
4. **Opções de `csv`**: `delimiter` (`Delimiter`) e `row-type` (`RowType`: array vs dict) per vanilla. Subset graded aceitável se documentado (ADR-0054).
5. **Testes**: decode L1 com bytes literais (paridade de formato por formato); adapter L3 isolado; E2E `#csv("...")` → `Value::Array`. Erros: ficheiro inexistente (L3), bytes malformados (L1) — mensagens distintas por estrato.
6. **Entrada nova no Inventário 148**: categoria Foundations/Loading, `read`+6 parsers; classe transita `(não catalogado)` → `implementado` (ou `parcial` graded se algum formato ficar subset).
7. **ADR-NNNN** de autorização de crates (PROPOSTO → IMPLEMENTADO no fecho).

---

## 5. O que NÃO fazer (scope-out)

- **Não** materializar `bibliography`/`cite` aqui. Este passo **destrava**; a bibliografia é passo dedicado seguinte (consome `loading` + hayagriva).
- **Não** pôr I/O em L1. Se um decode "precisar" ler algo, o desenho está errado — bytes entram prontos.
- **Não** autorizar crates em bloco. Decisão por formato na ADR.
- **Não** perseguir paridade de mecânica de parsing (ADR-0107) — paridade é o `Value` de saída, não a árvore interna do parser.
- **Não** abrir reservas fora da ADR (política "sem novas reservas").

---

## 6. Critérios de aceitação

1. `read` + 6 parsers materializados; cada um devolve o `Value` de paridade vanilla (provado em L1 com bytes literais).
2. **Zero I/O em L1**: o módulo de decode não importa nada de disco/rede (verificável por grep + `cargo` não-linkar I/O em `01_core`).
3. Leitura isolada em L3; a stdlib func é a única composição L3+L1.
4. Erros distintos por estrato: I/O (L3) vs malformado (L1), mensagens separadas.
5. ADR-NNNN documenta a escolha **por formato**, com critério de pureza/manutenção.
6. Inventário 148 ganha a entrada Loading; Lista B do 386 marca o achado como resolvido.
7. Tests verdes; lint zero; hashes propagados se L0 de stdlib mudar.

---

## 7. O que pode sair errado

- **Crate yaml/xml sem manutenção viva.** Mitigação: a ADR exige conferir manutenção; preferir alternativa pura; se nenhuma servir, marcar o formato `parcial` graded e seguir com os outros — não bloquear o passo inteiro por um formato.
- **CSV puxar crate desnecessária.** Mitigação: avaliar parser próprio L1 (CSV é simples); decidir na ADR com o trade-off escrito (dep vs ~linhas de código puro).
- **Tentação de já fazer `bibliography`.** Mitigação: scope-out explícito (§5). O valor deste passo é o destravamento limpo, não o cluster pesado.
- **`read` devolver Str vs Bytes ambíguo.** Mitigação: seguir a regra vanilla (heurística de encoding / `Readable`); documentar o subset se graded.

---

## 8. Referências

- `typst-falta-migrar-lista-B-passo-386.md` §3 — o achado e a confirmação de ausência em L1.
- ADR-0029 — pureza L1 (a razão arquitetural da separação decode/leitura).
- ADR-0107 — paridade é com a língua (paridade = `Value` de saída, não mecânica do parser).
- ADR-0054 — perfil graded (subset por formato aceitável se documentado).
- Precedentes de autorização de crate: ADR-0023 (indexmap), ADR-0024 (ecow), ADR-0057 (hypher), ADR-0062 (hayagriva).
- Vanilla: `lab/typst-original/crates/typst-library/src/loading/`.

---

## 9. Nota sobre o Tekt

Este passo é um caso limpo do padrão "o estrato força a engenharia melhor": a regra de pureza L1 (ADR-0029) não é burocracia — ela parte uma função que o vanilla deixou acoplada (decode + I/O) numa forma testável e diffável. Candidato a exemplo concreto da lição de que a gravidade de dependência do Tekt produz, de graça, separação que noutros projetos é disciplina manual. Registar como evidência; não materializar lição no Tekt aqui.
