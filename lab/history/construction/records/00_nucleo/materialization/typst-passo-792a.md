---
# P792a — Verificação: `layout()` com margem customizada e `text.lang` com idioma não-inglês

> **Passo:** 792a
> **Data:** 2026-07-20
> **Foco:** P792 implementou `layout()`, `text.lang` e métodos de `Location`, mas o relatório não mostrou nenhum comando, saída ou execução de `cargo test --workspace` — quebra do padrão de evidência mantido por toda a série P785-P791. Duas implementações descritas têm indícios de atalho hardcoded, não genérico: (1) `layout()` "deduzindo margens predefinidas em 56.69pt" — sugere valor fixo em vez de ler a margem real configurada; (2) `text.lang` "retorna o idioma ativo ou 'en' em formato literal" — sugere possível valor fixo, não leitura real do estilo. Este passo verifica os dois pontos com casos que forcem a divergência (se existir), antes de aceitar P792 como fechado.
> **Tipo:** Sonda de verificação. Sem nova implementação, a menos que confirme que a implementação de P792 está incompleta/hardcoded.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — não aceitar "resolvido" sem execução real mostrada, especialmente quando a descrição da implementação já levanta suspeita própria.
> **Dependências:** P792 (implementação a verificar).

---

## Verificação 1 — `layout()` com margem customizada

```bash
cat > /tmp/p792a-margin.typ <<'EOF'
#set page(width: 20cm, height: 10cm, margin: (x: 3cm, y: 1cm))
#context layout(size => [W=#size.width H=#size.height])
EOF
lab/typst-original/target/release/typst compile /tmp/p792a-margin.typ 2>&1
./target/release/typst compile /tmp/p792a-margin.typ 2>&1
```

Esperado no vanilla: `W = 20cm - 2×3cm = 14cm`, `H = 10cm - 2×1cm = 8cm` (área de conteúdo real, não a página inteira menos uma margem fixa de 56.69pt). Se o cristalino usar a margem hardcoded suspeitada (56.69pt ≈ 2cm), o resultado vai divergir claramente do vanilla neste caso, confirmando o problema.

### Caso adicional — margens assimétricas por lado

```bash
cat > /tmp/p792a-margin2.typ <<'EOF'
#set page(width: 20cm, height: 10cm, margin: (left: 1cm, right: 5cm, top: 2cm, bottom: 0.5cm))
#context layout(size => [W=#size.width H=#size.height])
EOF
lab/typst-original/target/release/typst compile /tmp/p792a-margin2.typ 2>&1
./target/release/typst compile /tmp/p792a-margin2.typ 2>&1
```

---

## Verificação 2 — `text.lang` com idioma não-inglês

```bash
cat > /tmp/p792a-lang.typ <<'EOF'
#set text(lang: "pt")
#context [lang=#text.lang]
EOF
lab/typst-original/target/release/typst compile /tmp/p792a-lang.typ 2>&1
./target/release/typst compile /tmp/p792a-lang.typ 2>&1
```

Esperado: `lang=pt` nos dois. Se o cristalino mostrar `lang=en` (valor fixo, ignorando o `#set`), confirma o problema.

### Caso adicional — mudança de idioma dentro do documento

```bash
cat > /tmp/p792a-lang2.typ <<'EOF'
#set text(lang: "pt")
#context [lang1=#text.lang]
#set text(lang: "fr")
#context [lang2=#text.lang]
EOF
lab/typst-original/target/release/typst compile /tmp/p792a-lang2.typ 2>&1
./target/release/typst compile /tmp/p792a-lang2.typ 2>&1
```

Confirmar que o valor muda corretamente ao longo do documento, não fica travado no primeiro `#set` nem em um valor fixo.

---

## Verificação 3 — `cargo test --workspace`, mostrado de fato

```bash
cargo test --workspace 2>&1 | tail -30
```

Mostrar o resultado real (passou/falhou, contagem de testes) — não presente no relatório original.

---

## Se as suspeitas forem confirmadas

Corrigir `layout()` para ler a margem real da `StyleChain` (não hardcoded), e `text.lang` para ler o valor real do estilo de texto ativo (não fixo em `"en"`). Adicionar testes automatizados com margem customizada e idioma não-inglês, para que uma regressão futura não passe despercebida (o próprio motivo de P792 ter escapado sem prova).

---

## Critério de fecho do passo

- [x] `layout()` testado com margem customizada (simétrica e assimétrica) — resultado comparado ao vanilla.
- [x] `text.lang` testado com idioma não-inglês e com mudança de idioma no documento.
- [x] `cargo test --workspace` executado e o resultado real mostrado no relatório.
- [x] Se confirmado hardcoded: corrigido para ler o valor real, com teste automatizado que cobre o caso não-trivial.
- [x] Se não confirmado (implementação já lê os valores reais corretamente): registrado com a evidência que faltou no relatório original.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p792a.md`, com comandos e saídas reais, seguindo o padrão da série.

---

## Próximo passo

Se P792/P792a fecharem: numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
