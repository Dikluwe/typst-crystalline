---
# P646 — Chave vazia em bibliografia YAML não produz erro

> **Passo:** 646
> **Data:** 2026-07-09
> **Foco:** P645 testou, de passagem, o mesmo caso de chave vazia que P644 corrigiu, mas usando um ficheiro `.yaml` em vez de `.bib`. O comando terminou com sucesso — a verificação de chave vazia não dispara para este formato. Isto foi registado como "fora do âmbito, investigação futura", mas é o próprio conserto de P644 incompleto para um dos dois formatos de bibliografia suportados, não um caso novo e independente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P644 (correcção original, aparentemente só cobrindo `.bib`), P645 (onde o gap em `.yaml` foi encontrado).

---

## Sonda

### Confirmar o caminho de código para cada formato

```bash
grep -n "\.bib\|\.yaml\|\.yml\|from_biblatex\|from_yaml" 01_core/src/engine/eval/bibliography.rs | head -30
```

Confirmar se ficheiros `.bib` e `.yaml`/`.yml` passam pelo mesmo código de validação de chave vazia, ou por caminhos completamente separados — se forem separados, a correcção de P644 pode ter sido aplicada só a um dos dois sem se dar por isso.

### Confirmar directamente com o vanilla

```bash
cat > /tmp/p646-chave-vazia.yaml <<'EOF'
"":
  type: Article
  title: Sem chave
  author: Alguém
  date: 2024
EOF
cat > /tmp/p646-chave-vazia.typ <<'EOF'
#bibliography("/tmp/p646-chave-vazia.yaml")
EOF
lab/typst-original/target/release/typst compile /tmp/p646-chave-vazia.typ /tmp/p646-vanilla.pdf
echo "Exit code vanilla: $?"
```

Confirmar se o vanilla também rejeita chave vazia em `.yaml`, com que mensagem.

### Critério de fecho da sonda

- [ ] Confirmado se `.bib` e `.yaml` passam por caminhos de código separados para a validação de chave vazia.
- [ ] Comportamento do vanilla confirmado para `.yaml` com chave vazia.

---

## Implementação

Aplicar a mesma verificação de chave vazia (já implementada por P644 para `.bib`) ao caminho de `.yaml`/`.yml`, com a mesma mensagem de erro, mantendo o transporte via `SourceDiagnostic` já corrigido por P645.

### Critério de fecho da implementação

- [ ] Chave vazia em `.yaml`/`.yml` produz erro, igual ao comportamento já corrigido para `.bib`.
- [ ] Teste directo confirmando o novo comportamento.
- [ ] Ficheiros `.yaml`/`.yml` válidos (chave preenchida) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p646-chave-vazia.typ /tmp/p646.pdf
echo "Exit code cristalino: $?"
```

Confirmar que agora falha, com mensagem equivalente à já produzida para `.bib`.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, causa confirmada.
- [ ] Chave vazia em `.yaml`/`.yml` produz erro.
- [ ] `.bib` continua a funcionar sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p646.md`, com hash do commit.

---

## Nota

Com isto, a sequência de falhas silenciosas iniciada em P633 fica genuinamente fechada — incluindo o caso que só apareceu como efeito colateral de testar P645 com um formato de ficheiro diferente do original.
