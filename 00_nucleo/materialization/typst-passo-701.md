---
# P701 — `cbor` como valor com `.encode`, `cbor(bytes)` além de caminho

> **Passo:** 701
> **Data:** 2026-07-10
> **Foco:** P700 confirmou que `cbor.encode()` (serializa `Value` para CBOR) e `cbor(bytes)` (decodifica bytes crus, não só caminho) estão em falta — o único bloqueio restante para os três consumidores de `cetz-core.wasm` (`aabb.typ`, `bezier.typ`, `matrix.typ`), todos passando pelo mesmo `call_wasm`. Este passo implementa o mínimo necessário para desbloquear isso, decidindo explicitamente se generaliza para `json`/`yaml`/`toml`/`xml` já agora ou faz scope-out para essas.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P700 (onde o bloqueio foi isolado com `file:line` do vanilla), P685 (padrão de tipo-como-valor-chamável, já usado para `int`/`float`/`str`/`type`, reaproveitável aqui).

---

## Sonda

### Confirmar a tabela exacta de conversão `Value` → CBOR no vanilla

```bash
grep -n "fn encode\|impl.*Value.*Cbor\|into_writer" lab/typst-original/crates/typst-library/src/loading/cbor.rs
```

Ler o código fonte directamente — confirmar como cada tipo de `Value` é serializado (símbolo → texto, `content` → mapa, outros → texto via `repr`, conforme P700 já descreveu de forma resumida — confirmar os detalhes exactos, `file:line`).

### Confirmar o comportamento de `cbor(bytes)` (decodificação) e `cbor.encode(value)` (codificação) directamente

```bash
cat > /tmp/p701-cbor.typ <<'EOF'
#let bytes_codificados = cbor.encode((a: 1, b: "texto", c: (1, 2, 3)))
#type(bytes_codificados)
#cbor(bytes_codificados)
EOF
lab/typst-original/target/release/typst compile /tmp/p701-cbor.typ /tmp/p701-vanilla.pdf
pdftotext /tmp/p701-vanilla.pdf -
```

Confirmar que codificar e depois decodificar produz o dicionário original (ida e volta), e testar casos de borda: símbolos, conteúdo (`content`), valores não triviais.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p701-cbor.typ /tmp/p701-cristalino.pdf
echo "Exit code: $?"
```

### Decidir o âmbito: só `cbor`, ou generalizar já para `json`/`yaml`/`toml`/`xml`

```bash
grep -n "native_loader!\|fn native_json\|fn native_yaml\|fn native_toml\|fn native_xml" 01_core/src/rules/stdlib/loading.rs
```

Confirmar quantos destes já partilham a mesma macro/estrutura (`native_loader!`) — se a mudança para aceitar `Bytes` for barata de generalizar a todos ao mesmo tempo (mesma macro, um só ponto de alteração), fazer isso agora evita repetir o mesmo trabalho depois para cada um. Se for mais complexo, registar scope-out explícito só para `cbor`.

### Critério de fecho da sonda

- [ ] Tabela de conversão `Value → CBOR` confirmada com `file:line`.
- [ ] Comportamento de ida-e-volta (`encode` → `cbor(bytes)`) confirmado contra o vanilla.
- [ ] Decisão de âmbito (só `cbor`, ou generalizado) tomada com base na estrutura real do código, não por conveniência.

---

## Implementação

### `cbor` como valor com campo `.encode`

Seguindo o padrão já estabelecido em P685 (tipos-como-valores-chamáveis), `cbor` passa a ser um valor com um método `encode` acessível por field access, mantendo `cbor(...)` como chamada directa (decodificação).

### `native_cbor` aceita `Bytes` além de caminho

Estender a resolução de `DataSource` (ou equivalente) para aceitar bytes directos, não só caminho de ficheiro — replicando o comportamento confirmado pela sonda.

### `cbor.encode(value)`

Serializar qualquer `Value` para CBOR, seguindo exactamente a tabela de conversão confirmada pela sonda.

### Critério de fecho da implementação

- [ ] `cbor.encode(value)` funciona para os tipos confirmados pela sonda.
- [ ] `cbor(bytes)` decodifica correctamente.
- [ ] Ida e volta (`cbor(cbor.encode(x)) == x`, para valores simples) confirmada.
- [ ] Decisão de âmbito (generalizado ou só `cbor`) documentada com razão.

---

## Validação

```bash
./target/release/typst /tmp/p701-cbor.typ /tmp/p701-depois.pdf
pdftotext /tmp/p701-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir exactamente a reprodução de P700

```bash
cat > /tmp/p701-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p701-cetz.typ /tmp/p701-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p701-cetz.png -r 150 /tmp/p701-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688. Se falhar: registar o próximo bloqueio com honestidade, mesma disciplina de sempre.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, tabela de conversão e comportamento confirmados contra o vanilla.
- [ ] `cbor.encode`/`cbor(bytes)` implementados e testados.
- [ ] Âmbito (só `cbor` ou generalizado) decidido e documentado.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p701.md`, com hash do commit.
