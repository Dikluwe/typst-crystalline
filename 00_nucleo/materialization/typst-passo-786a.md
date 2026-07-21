---
# P786a — Erros de sintaxe descartados silenciosamente (exit 0 com sintaxe inválida)

> **Passo:** 786a
> **Data:** 2026-07-20
> **Foco:** Um relatório de reverificação independente (gerado fora do fluxo desta conversa, por outra ferramenta/sessão) reportou que `#let x = (` — delimitador não fechado, sintaxe claramente inválida — compila com sucesso no cristalino (exit 0, gera um PDF de 1897 bytes), enquanto o vanilla erra corretamente (`error: unclosed delimiter`, exit 1). A causa apontada, ainda não verificada por esta sessão: `01_core/src/engine/eval/mod.rs:308-321` só propaga `InvalidHexNumber`/`InvalidUnicodeCodepoint` como erro fatal; outros tipos de erro sintático do parser são descartados. Este passo primeiro reconfirma o achado de forma independente (não aceitar às cegas, dado que a proveniência do relatório original é externa a este fluxo) e depois corrige.
> **Tipo:** Sonda de reconfirmação + Implementação directa (urgente — pode afetar a validade de testes anteriores de toda a série).
> **Tamanho:** M/L — mexe no caminho central de propagação de erros do parser para o avaliador.
> **ADR-0108 EM VIGOR** — reconfirmar antes de aceitar, dado que o achado não veio de um passo desta conversa com disciplina já verificada.
> **Prioridade:** Urgente — se confirmado, qualquer teste anterior desta série que não tenha verificado explicitamente "o que acontece com sintaxe inválida" pode ter passado por um caminho que mascarava erros.
> **Dependências:** Relatório de reverificação externo (T1, T2 — não commitado nesta sessão, achado a reconfirmar).

---

## Passo 0 — Reconfirmar o achado de forma independente

```bash
cat > /tmp/p786a-unclosed.typ <<'EOF'
#let x = (
EOF
lab/typst-original/target/release/typst compile /tmp/p786a-unclosed.typ 2>&1
echo "exit vanilla: $?"
./target/release/typst compile /tmp/p786a-unclosed.typ /tmp/p786a-cristalino.pdf 2>&1
echo "exit cristalino: $?"
ls -la /tmp/p786a-cristalino.pdf 2>&1
```

Confirmar exit code e se um PDF é de fato gerado. Se confirmado: prosseguir. Se não reproduzir: o achado externo pode estar desatualizado ou incorreto — documentar a discrepância e não prosseguir com a implementação sem entender por quê.

### Reconfirmar também T1 (warnings sintáticos ausentes, mesmo relatório externo)

```bash
cat > /tmp/p786a-stars.typ <<'EOF'
**
EOF
lab/typst-original/target/release/typst compile /tmp/p786a-stars.typ 2>&1
./target/release/typst compile /tmp/p786a-stars.typ 2>&1
```

---

## Sonda — mecanismo exato

```bash
grep -n "InvalidHexNumber\|InvalidUnicodeCodepoint\|SyntaxError\|errors_and_warnings" 01_core/src/engine/eval/mod.rs 01_core/src/engine/syntax/*.rs 2>/dev/null
```

Confirmar:
1. Que tipos de erro sintático o parser do cristalino já produz internamente (mesmo que não propagados).
2. Onde exatamente a filtragem/descarte acontece — o `match`/condição que só propaga os dois tipos citados.
3. Como o vanilla trata isso (`typst_syntax::node::errors_and_warnings`, referido no achado externo) — propaga todo erro sintático, sem filtro por tipo.

```bash
grep -n "fn errors_and_warnings\|SyntaxKind::Error" lab/typst-original/crates/typst-syntax/src/node.rs 2>/dev/null | head -20
```

---

## Implementação

Corrigir `eval/mod.rs` (ou o ponto exato confirmado pela sonda) para propagar **todo** erro sintático do parser como `SourceDiagnostic::error`, não só os dois tipos hardcoded. Confirmar também se há warnings sintáticos (T1) que devem ser propagados como `SourceDiagnostic::warning`, não só erros.

---

## Validação

```bash
./target/release/typst compile /tmp/p786a-unclosed.typ 2>&1
echo "exit: $?"
```

Confirmar exit 1, erro reportado, nenhum PDF gerado.

```bash
./target/release/typst compile /tmp/p786a-stars.typ 2>&1
```

Confirmar warning presente (se T1 for confirmado como caso real).

### Regressão — não quebrar documentos válidos

```bash
cargo test --workspace
crystalline-lint .
```

Esta é a validação mais crítica deste passo: propagar mais tipos de erro sintático pode, por engano, começar a rejeitar documentos que antes compilavam corretamente por acidente (o próprio bug pode ter mascarado outros problemas em testes existentes). Investigar qualquer teste que passe a falhar — não assumir que é regressão automaticamente; pode ser um documento de teste que sempre teve sintaxe questionável e só "funcionava" por causa deste bug.

---

## Critério de fecho do passo

- [ ] Achado T2 reconfirmado de forma independente (comando + saída mostrados, não herdado do relatório externo sem verificação).
- [ ] Achado T1 (warnings) reconfirmado ou descartado.
- [ ] Mecanismo exato identificado por leitura de código.
- [ ] Correção propaga todo erro sintático (e warning, se aplicável) do parser.
- [ ] `#let x = (` agora erra corretamente, sem gerar PDF.
- [ ] Qualquer teste que passe a falhar investigado individualmente — não assumido como regressão nem ignorado.
- [ ] `cargo test --workspace` verde (ou falhas investigadas e justificadas uma a uma).
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p786a.md`.

---

## Próximo passo

Retomar P786 (triagem em lote, próximos 15 módulos) ou continuar com os outros achados do mesmo relatório externo (T4 — mensagem errada para main não-UTF8; T6 — spans `<detached>` em `eval()`) conforme prioridade.
