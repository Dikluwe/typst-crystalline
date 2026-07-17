---
# P690 — Unificar indexação de `str` para bytes, corrigindo inconsistência interna

> **Passo:** 690
> **Data:** 2026-07-10
> **Foco:** P689 descobriu, como efeito colateral, que `str.len()`/`.at()`/`.slice()` do cristalino indexam por carácter, enquanto o vanilla — e os métodos `position()`/`match()` que o próprio P689 acabou de adicionar, correctamente indexados por byte — usam byte. Isto deixa o cristalino com uma API de `str` internamente inconsistente: combinar `position(...)` com `at(...)` no mesmo texto não-ASCII produz resultado errado, silenciosamente. Este é um problema de correcção da linguagem, não um detalhe de implementação, e afecta qualquer documento com texto acentuado ou fora de ASCII.
> **Tipo:** Sonda + Implementação. Prioridade alta.
> **Tamanho:** L. Toca todos os métodos de `str` já existentes, potencialmente muitos call-sites.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança fundamental na semântica de um tipo usado em todo o lado; sonda obrigatória e cuidado redobrado antes de qualquer código.

---

## Contexto

Confirmado por P689: `"café".len()` dá `4` no cristalino (conta caracteres: c-a-f-é), `5` no vanilla (conta bytes: c-a-f-é ocupa 2 bytes em UTF-8, mais 3 de "caf" = 5). O mesmo desalinhamento afecta `.at(i)` e `.slice(...)`.

Isto não é só uma diferença de números — é uma inconsistência **dentro do próprio cristalino**: `position()`/`match()` (P689) já indexam por byte, correctamente. `len()`/`at()`/`slice()` continuam a indexar por carácter. Um utilizador (ou um pacote, como `oxifmt`/`cetz`) que use os dois tipos de método sobre o mesmo texto obtém índices que não correspondem uns aos outros.

---

## Sonda

### Confirmar o alcance completo da inconsistência

```bash
cat > /tmp/p690-inconsistencia.typ <<'EOF'
#let s = "café mais texto"
#s.len()
#s.position("m")
#s.at(s.position("m"))
EOF
lab/typst-original/target/release/typst compile /tmp/p690-inconsistencia.typ /tmp/p690-vanilla.pdf
pdftotext /tmp/p690-vanilla.pdf -

./target/release/typst /tmp/p690-inconsistencia.typ /tmp/p690-cristalino.pdf
pdftotext /tmp/p690-cristalino.pdf -
```

Confirmar directamente que `s.at(s.position("m"))` dá "m" no vanilla, mas dá um carácter diferente (ou erro) no cristalino — a prova concreta do problema, não só a diferença de `len()`.

### Listar todos os métodos de `str` e confirmar qual convenção cada um usa hoje

```bash
grep -n "fn str_\|\"at\"\|\"slice\"\|\"len\"\|\"position\"\|\"match\"\|\"codepoints\"\|\"clusters\"" 01_core/src/engine/stdlib/collections.rs
```

Para cada método, confirmar contra o vanilla se indexa por byte ou por carácter — não assumir que só `at`/`slice`/`len` estão errados; `clusters()` e outros podem ter a mesma questão.

### Confirmar todos os call-sites internos que dependem da convenção actual (por carácter)

```bash
grep -rn "\.len()\|\.at(\|\.slice(" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -i "str\|text" | grep -v test | wc -l
```

Confirmar quantos sítios do próprio código do cristalino (não só código de utilizador) já dependem da convenção actual — mudar a semântica pode ter efeitos em cascata internos, não só externos.

### Critério de fecho da sonda

- [ ] Prova concreta da inconsistência confirmada (`s.at(s.position(...))` a dar resultado errado).
- [ ] Todos os métodos de `str` classificados: byte ou carácter, cada um confirmado contra o vanilla.
- [ ] Alcance de call-sites internos que dependem da convenção actual, mapeado.

---

## Implementação

Duas partes distintas, com prioridades diferentes.

### Parte obrigatória — `len()`, `at()`, `slice()` passam a indexar por byte

Estes são os nomes que o vanilla usa, com o significado do vanilla. Não é negociável: um método com o mesmo nome que o vanilla tem de se comportar como o vanilla (regra estabelecida em P662-P664). Migrar para indexação por byte, seguindo exactamente a convenção já confirmada para `position()`/`match()` em P689.

### Parte nova, não urgente — métodos de indexação por carácter, com nome próprio

Em vez de descartar a funcionalidade actual (indexação por carácter, genuinamente útil para texto humano), preservá-la sob nomes que não colidam com o vanilla — por exemplo `char-len()`, `char-at()`, `char-slice()`. Isto segue o mesmo padrão já usado para `table.numbering` (P459): uma capacidade a mais do cristalino, com nome próprio, claramente não-portável para o Typst real, documentada como tal.

Esta parte não bloqueia nem atrasa a parte obrigatória — pode ficar para um passo separado, sem pressa, "depois achamos uso para a contagem de caracteres" (usos plausíveis: contagem de palavras/caracteres visíveis para efeitos de limite de texto, validação de formulários, etc., a confirmar quando a necessidade aparecer).

### Critério de fecho da implementação

- [ ] `len()`, `at()`, `slice()` migrados para indexação por byte, confirmada contra o vanilla, um a um.
- [ ] `s.at(s.position(...))` produz o resultado correcto, testado com texto não-ASCII.
- [ ] Testes de regressão para texto ASCII puro (onde char == byte, a mudança não deve ter efeito visível) e para texto com acentos/multi-byte (onde a mudança é a correcção em si).
- [ ] Funcionalidade de indexação por carácter preservada sob nomes novos (`char-len`, `char-at`, `char-slice`), não descartada.
- [ ] Nomes novos registados como extensão cristalina, com razão escrita, seguindo o padrão de `table.numbering` (P459) e a regra de P662-P664 (diferença de linguagem só aceitável com nome distinto e decisão consciente).

---

## Validação

```bash
./target/release/typst /tmp/p690-inconsistencia.typ /tmp/p690-depois.pdf
pdftotext /tmp/p690-depois.pdf -
```

Comparar com o vanilla, confirmando `s.at(s.position("m"))` == "m" nos dois lados.

```bash
cargo test --workspace
crystalline-lint .
```

Correr o corpus geral de testes, com atenção especial a qualquer documento com texto acentuado ou não-ASCII já usado ao longo desta conversa (RTL, devanágari, etc.) — confirmar que nenhum desses regride com esta mudança de convenção.

---

## Critério de fecho do passo

- [ ] Sonda completa, alcance da inconsistência confirmado com prova concreta.
- [ ] Todos os métodos de `str` unificados para indexação por byte.
- [ ] Testado com ASCII puro (sem efeito) e com texto multi-byte (correcção confirmada).
- [ ] Call-sites internos revistos, sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p690.md`, com hash do commit.
- [ ] Se o alcance for maior do que um passo consegue cobrir com segurança: dividido em passos menores, com plano explícito, não forçado.
