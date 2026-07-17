---
# P772b (lote 4) — Varredura da stdlib: `typst_syntax::span`

> **Passo:** 772b (continuação da série P765a/P765b/P772/P772a)
> **Data:** 2026-07-16
> **Foco:** P772a fechou `pdf::accessibility` sem bugs reais (scope-out documentado + feature flags não presentes no vanilla padrão). Este passo classifica `typst_syntax::span` (10 itens na recontagem de P772a), o próximo por tamanho. `Span` é o mecanismo interno de rastreamento de posição no código-fonte (usado para mensagens de erro/diagnóstico) — potencial candidato a ter efeito observável via texto de mensagens de erro (regra do CLAUDE.md: "mensagem de erro é observável legítimo"), mesmo sendo infra-estrutura de parsing.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — atenção a efeito observável via mensagens de erro/diagnóstico, não só sintaxe da língua directamente.
> **Dependências:** P772a (metodologia, lição de não descartar módulo por categoria sem verificar item a item).

---

## Sonda — classificar os 10 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /typst_syntax::span/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt
```

Para cada item, ler o código-fonte do vanilla e confirmar:

```bash
grep -n "<item>" lab/typst-original/crates/typst-syntax/src/span.rs 2>/dev/null
```

E o estado actual no cristalino:

```bash
grep -rn "<item>\|struct Span\|SourceId" 01_core/src/entities/*.rs 01_core/src/rules/eval/*.rs 2>/dev/null
```

Atenção especial: `Span` normalmente inclui um identificador de ficheiro/pacote codificado nos bits altos (para diferenciar spans de `#import`s de pacotes diferentes) — confirmar se o cristalino replica esse mecanismo, e se a ausência dele teria efeito em mensagens de erro que apontam para código dentro de um pacote importado (ex: um erro dentro de `@preview/cetz` deveria apontar para o ficheiro certo do pacote, não para o documento principal).

```bash
cat > /tmp/p772b-span-test.typ <<'EOF'
#import "@preview/cetz:0.5.2": canvas
#canvas(body_com_erro_proposital)
EOF
lab/typst-original/target/release/typst compile /tmp/p772b-span-test.typ 2>&1
./target/release/typst compile /tmp/p772b-span-test.typ 2>&1
```

Comparar se a mensagem de erro aponta para o ficheiro/linha correcto nos dois casos.

---

## Implementação

Só para achados confirmados como bug real (mensagem de erro ou comportamento observável diverge). Corrigir com teste comparando a mensagem exacta, não só a presença de erro.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 10 itens de `typst_syntax::span` classificados item a item.
- [ ] Verificado especificamente se mensagens de erro dentro de pacotes importados apontam para o local correcto.
- [ ] Bugs reais corrigidos com teste de mensagem exacta.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772b.md`.

---

## Próximo passo

Continuar para `typst_syntax::package` neste mesmo lote (P772c) ou seguinte, conforme a divisão de trabalho.
