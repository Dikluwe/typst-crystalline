---
# P700 — Validação real: `cetz_core.wasm` e `#import "@preview/cetz:0.5.2"`

> **Passo:** 700
> **Data:** 2026-07-10
> **Foco:** Último dos passos planeados por P696 para o âmbito mínimo (plugins puros). Testa o suporte a plugins WASM, agora funcional e validado com sintaxe real (P699/P699b), contra o plugin real que motivou todo este trabalho — `cetz_core.wasm`. Mede até onde `#import "@preview/cetz:0.5.2"` avança agora, com a mesma disciplina de honestidade já usada em P686-P694 (registar o próximo bloqueio, não assumir sucesso).
> **Tipo:** Sonda + validação directa. Correcção pontual se algo pequeno falhar.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Compilar sempre um documento `.typ` real, não só testes unitários (lição reforçada por P699b).
> **Dependências:** P699/P699b (plugin funcional, validado com sintaxe real), P696 (mapa do protocolo, confirmação de que `cetz` só usa chamadas puras).

---

## Sonda

### Chamar uma função real de `cetz_core.wasm` directamente

```bash
find ~/.cache/typst/packages/preview/cetz/0.5.2 -name "*.wasm"
grep -n "plugin(" ~/.cache/typst/packages/preview/cetz/0.5.2/src/aabb.typ
```

Confirmar o nome de uma função exportada simples (por exemplo, algo em `aabb.typ`, já identificado por P696 como um dos consumidores do plugin), e chamá-la isoladamente, fora do resto de `cetz`, para confirmar que o cristalino consegue invocar o plugin real antes de testar o pacote inteiro.

```bash
cat > /tmp/p700-cetz-core.typ <<'EOF'
#let core = plugin("/cetz-core/cetz_core.wasm")
#(type(core))
EOF
```

Ajustar o caminho conforme a estrutura real do pacote (pode ser necessário simular a raiz do pacote, ou testar a partir de dentro do próprio `cetz`, dado que o caminho é `/cetz-core/...`, absoluto à raiz do pacote, já confirmado por P686/P696).

### Confirmar `#import "@preview/cetz:0.5.2"` de novo, com o documento e padrão de P688

```bash
cat > /tmp/p700-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p700-cetz.typ /tmp/p700-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p700-cetz.png -r 150 /tmp/p700-cetz.pdf 2>/dev/null
```

### Critério de fecho da sonda

- [ ] Chamada isolada a uma função real de `cetz_core.wasm` confirmada a funcionar.
- [ ] `cetz` completo re-testado, com o próximo estado registado com honestidade — sucesso completo, ou próximo bloqueio identificado.

---

## Decisão

Se `cetz` produzir PDF completo: comparar visualmente com a imagem do vanilla já descrita por P688 (linha diagonal + círculo). Se coincidir, a cadeia de validação de pacotes iniciada em P678 fica finalmente fechada, com um pacote real da comunidade a funcionar de ponta a ponta.

Se houver mais um bloqueio: registar com a mesma disciplina de sempre, sem forçar uma correcção improvisada só para "fechar" — decidir se vale a pena mais um passo, ou se é boa altura para parar esta cadeia longa (P678-700) e voltar a ela mais tarde.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Medir também o tempo de compilação de `cetz` (se produzir PDF), dado que P698 assinalou "instância fresca por chamada" como potencial custo — confirmar se isso é perceptível na prática ou irrelevante para este caso.

```bash
time ./target/release/typst /tmp/p700-cetz.typ /tmp/p700-timed.pdf
```

---

## Critério de fecho do passo

- [ ] Chamada isolada a `cetz_core.wasm` confirmada.
- [ ] `cetz` re-testado com honestidade — sucesso completo (com comparação visual) ou próximo bloqueio registado.
- [ ] Se sucesso: tempo de compilação medido, para confirmar se "instância fresca por chamada" (P698) é um problema prático ou não.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p700.md`, com hash do commit.
- [ ] Estado da cadeia P678-700 declarado com precisão — fechada, ou com próximo passo claro.
