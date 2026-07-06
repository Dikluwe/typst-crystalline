---
# P578 — Instrumentar `flush_line()` para confirmar a causa da sobreposição RTL

> **Passo:** 578
> **Data:** 2026-07-05
> **Foco:** P577 mediu uma sobreposição real de glifos quando uma linha árabe tem uma palavra latina ou um número no meio, e propôs uma explicação — `flush_line()` a ser chamado duas vezes para o que devia ser uma linha só, cada chamada a alinhar independentemente à margem direita. A explicação foi marcada como inferência, não confirmada. Este passo instrumenta o código para confirmar ou refutar, antes de qualquer correcção.
> **Tipo:** Sonda directa, com instrumentação temporária. Sem correcção de código nesta passagem.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Não corrigir `align_current_line_rtl()` ou `layout_chunk` sem confirmar primeiro qual das duas é a causa, ou se é outra coisa.
> **Dependências:** P577 (onde a sobreposição foi medida e a hipótese proposta), P576 (código de `align_current_line_rtl`, ainda com estado revisto para "implementação parcial").

---

## Contexto

P577 mediu, para o documento `الكتاب 42 على الطاولة`: duas linhas registadas pelo `pdftotext` onde devia haver uma, ordem de palavras trocada, e sobreposição visual confirmada com `mutool draw`. A hipótese: o mecanismo de decidir se uma linha está cheia (`right_margin = width - margin`, pensado para LTR) reage de forma diferente quando a direcção do texto muda a meio, disparando um `flush_line()` antes do previsto — e cada uma das duas chamadas resultantes alinha o seu grupo de palavras à margem direita, independentemente uma da outra, produzindo sobreposição.

---

## Instrumentação

Adicionar, temporariamente, um contador e um registo do conteúdo de `current_line` a cada chamada de `flush_line()`, só para este passo — não para ficar no código final:

```rust
// Temporário, só para P578:
eprintln!("flush_line chamado. current_line tem {} items: {:?}",
    self.regions.current.current_line.len(),
    self.regions.current.current_line.iter().map(|i| i.plain_text()).collect::<Vec<_>>());
```

Colocar isto no início de `flush_line()`, antes de qualquer outra lógica.

### Teste

```bash
cargo build --release --bin typst
cat > /tmp/p578-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p578-rtl.typ /tmp/p578.pdf 2> /tmp/p578-log.txt
cat /tmp/p578-log.txt
```

Ler o registo produzido. Confirmar:

1. Quantas vezes `flush_line()` é chamado para este documento de uma linha.
2. Se for chamado mais de uma vez, o que está em `current_line` em cada chamada — confirma ou não a hipótese de que o texto foi dividido em dois grupos.
3. Se a hipótese estiver certa, confirmar também porque a decisão de "a linha está cheia" foi accionada antes do fim do texto — que largura foi calculada, e que largura estava disponível nesse momento.

### Comparar com o caso sem direcção mista

Repetir o mesmo registo com o documento que P577 confirmou como correcto (`الكتاب على الطاولة`, sem o `42`):

```bash
cat > /tmp/p578-rtl-puro.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب على الطاولة
EOF
./target/release/typst /tmp/p578-rtl-puro.typ /tmp/p578-puro.pdf 2> /tmp/p578-puro-log.txt
cat /tmp/p578-puro-log.txt
```

Confirmar que, neste caso, `flush_line()` só é chamado uma vez — isso reforça que a diferença está mesmo ligada à presença da direcção mista, não a outra coisa qualquer.

### Critério de fecho

- [ ] Número de chamadas a `flush_line()` confirmado para os dois documentos.
- [ ] Conteúdo de `current_line` em cada chamada registado.
- [ ] Hipótese de P577 confirmada ou refutada, com o registo como prova.
- [ ] Se confirmada: localizado o ponto exacto (`file:line`) onde a decisão de quebra de linha é accionada de forma incorrecta para texto de direcção mista.
- [ ] Instrumentação removida depois deste passo — não fica no código final.

---

## Se a hipótese for confirmada

Não implementar a correcção neste passo. Escrever um passo novo, com a causa exacta já confirmada, para decidir a forma certa de corrigir — por exemplo, calcular a largura da linha usando a largura real de cada palavra na direcção final, não assumindo LTR até ao fim, antes de decidir se a linha está cheia.

## Se a hipótese for refutada

Registar o que o registo mostrou de facto, e propor uma nova hipótese com base nisso — não repetir a mesma explicação sem prova, o que já aconteceu antes nesta sequência (P566).

---

## Critério de fecho do passo

- [ ] Instrumentação feita, registo obtido para os dois casos.
- [ ] Hipótese de P577 confirmada ou refutada com prova, não com suposição.
- [ ] Instrumentação removida.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p578.md`, com o hash do commit, seguindo a regra de proveniência.
- [ ] Próximo passo (correcção) só especificado depois deste, com a causa exacta, não antes.
