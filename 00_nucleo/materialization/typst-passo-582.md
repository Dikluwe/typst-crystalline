---
# P582 — Varrer o resto do código por `font_size_pt` estático + corrigir a afirmação de paridade completa

> **Passo:** 582
> **Data:** 2026-07-05
> **Foco:** Duas coisas. Primeiro: o P579 real corrigiu `font_size_pt` estático em seis ou mais ficheiros (`cursor.rs`, `mod.rs`, `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`), encontrados um a um à medida que cada regressão aparecia. Não há garantia de que essa caça encontrou todos os sítios — só os que quebraram testes existentes. Este passo procura de forma sistemática, não reactiva. Segundo: o relatório final da sequência afirma "paridade de linguagem completa com o Typst 0.15.0", o que contradiz a lista de itens ainda ausentes já construída nesta conversa. Corrigir isso.
> **Tipo:** Verificação sistemática + Correcção de documentação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Procurar de forma completa, não esperar que o próximo teste a falhar revele o próximo sítio.
> **Dependências:** P579 real, P580 (a correcção espalhada por vários ficheiros), P578 (diagnóstico inicial, parcialmente incorrecto — avanço zero não bate com o delta de 16,13pt medido).

---

## Parte 1 — Procura sistemática por `font_size_pt` estático

### Não confiar na caça reactiva já feita

```bash
grep -rn "font_size_pt" 01_core/src/ 03_infra/src/ --include="*.rs"
```

Para cada ocorrência encontrada, confirmar se já foi corrigida (usa `style.size` ou equivalente dinâmico) ou se ainda usa o valor estático directamente. Não assumir que os seis ficheiros já tocados são todos os que precisavam de mudar — a lista foi construída à medida que os testes falhavam, não por uma procura completa desde o início.

### Testar tamanhos de fonte extremos, não só 40pt

```bash
cat > /tmp/p581-tamanhos.typ <<'EOF'
#set text(size: 8pt)
oito oito oito oito oito oito oito oito oito oito oito oito

#set text(size: 72pt)
setenta e dois setenta e dois setenta

#set text(size: 6pt)
seis seis seis seis seis seis seis seis seis seis seis seis seis seis seis
EOF
./target/release/typst /tmp/p581-tamanhos.typ /tmp/p581.pdf
mutool draw -o /tmp/p581.png -r 150 /tmp/p581.pdf
```

Confirmar visualmente que nenhum destes tamanhos, incluindo os que nunca foram testados antes (8pt, 72pt, 6pt), produz sobreposição.

### Testar dentro de estruturas que ainda podem não ter sido tocadas

```bash
cat > /tmp/p581-estruturas.typ <<'EOF'
#table(
  columns: 2,
  [#set text(size: 30pt); célula um linha um linha dois],
  [normal],
)

#figure(
  [#set text(size: 30pt); dentro de uma figura, texto grande que pode quebrar linha]
)
EOF
./target/release/typst /tmp/p581-estruturas.typ /tmp/p581-estruturas.pdf
mutool draw -o /tmp/p581-estruturas.png -r 150 /tmp/p581-estruturas.pdf
```

Tabelas e figuras usam sub-layouts próprios — confirmar se estes também foram cobertos pela correcção, ou se são mais dois sítios ainda por encontrar.

### Critério de fecho da Parte 1

- [ ] Todas as ocorrências de `font_size_pt` no código listadas e classificadas — corrigida ou ainda estática.
- [ ] Tamanhos extremos (8pt, 72pt, 6pt) testados sem sobreposição.
- [ ] Estruturas com sub-layout (tabela, figura) testadas especificamente.
- [ ] Se for encontrado mais algum sítio: corrigido aqui, não deixado para o próximo teste que falhar revelar por acidente.

---

## Parte 2 — Corrigir a afirmação de paridade completa

O relatório da sequência afirma "paridade de linguagem completa com o Typst 0.15.0". Isto não está de acordo com o inventário já construído nesta conversa, que lista como ausentes: exportação HTML, SVG, PNG; IDE/LSP; fontes de cor para emoji; fontes Type1; escrita vertical CJK; quebra de linha CJK/Thai; PDF Tagged/UA; compressão por object streams; stream de metadados XMP; pacotes `@preview`; entre outros.

### Acção

Corrigir o relatório onde essa afirmação aparece. A frase correcta distingue: paridade de sintaxe e semântica testada dentro do que já foi coberto (confirmada por muitos passos ao longo desta conversa) não é o mesmo que "completa" sem qualificação — há áreas nunca implementadas, listadas no inventário, que continuam ausentes.

### Critério de fecho da Parte 2

- [ ] A frase "paridade de linguagem completa" localizada e substituída por uma descrição que distingue o que está confirmado do que continua ausente.
- [ ] Referência directa ao inventário (`inventario-decisoes-pendentes.md`) para a lista completa, em vez de repetir a lista aqui.

---

## Critério de fecho do passo

- [ ] Parte 1: procura sistemática feita, não reactiva; tamanhos extremos e sub-layouts testados.
- [ ] Parte 2: afirmação de paridade completa corrigida.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p582.md`, com hash do commit.
- [x] O passo "Estabilização das Métricas" já recebeu número próprio (P580, registo retroactivo) — não fica sem identificação.
