---
# P577 — Confirmar `cargo test --workspace` e medir posições depois do alinhamento RTL

> **Passo:** 577
> **Data:** 2026-07-05
> **Foco:** P576 declarou "CONCLUÍDO" sem o resultado de `cargo test --workspace`, que a própria secção 6 do relatório diz estar "em background". O critério de fecho do passo pedia esse resultado antes de fechar. Este passo confirma isso, e acrescenta a medição de posições por palavra que os passos anteriores da sequência RTL sempre usaram, aqui ainda em falta.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Um passo não fica fechado com uma verificação obrigatória pendente, mesmo que o resto pareça correcto.
> **Dependências:** P576 (onde a implementação foi feita, sem esta confirmação).

---

## Parte 1 — Confirmar `cargo test --workspace`

```bash
cargo test --workspace
```

Não avançar para mais nada até este resultado estar confirmado, com o número de testes passados e falhados, não "em background".

### Critério de fecho da Parte 1

- [ ] Resultado completo obtido, não pendente.
- [ ] Se houver falhas: confirmar se são as mesmas já conhecidas de antes de P576 (registando o hash do commit usado para comparar, seguindo a regra já escrita), ou novas.

---

## Parte 2 — Medir posições depois do alinhamento

```bash
cat > /tmp/p577-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب على الطاولة
EOF
./target/release/typst /tmp/p577-rtl.typ /tmp/p577.pdf
pdftotext -tsv /tmp/p577.pdf /tmp/p577.tsv
lab/typst-original/target/release/typst compile /tmp/p577-rtl.typ /tmp/p577-vanilla.pdf
pdftotext -tsv /tmp/p577-vanilla.pdf /tmp/p577-vanilla.tsv
```

Construir a mesma tabela de posições (esquerda, topo, largura, por palavra) já usada em P563/P564/P566/P567/P569/P574. Comparar directamente com a medição de P567 para o mesmo tipo de documento, agora com `dir: rtl` explícito, confirmando que:

1. O deslocamento para a margem direita está correcto (a primeira palavra da linha fica perto da margem direita da página, não da esquerda).
2. As posições relativas entre palavras (a ordem e o espaçamento, já corrigidas em P564/P567) continuam correctas depois do deslocamento inteiro da linha.

### Critério de fecho da Parte 2

- [ ] Tabela de posições construída e comparada com o vanilla, com números.
- [ ] Confirmado que o deslocamento para a direita não desfez a correcção de posições relativas já feita antes.

---

## Critério de fecho do passo

- [ ] Parte 1: `cargo test --workspace` confirmado, não pendente.
- [ ] Parte 2: posições medidas com a mesma tabela já usada antes na sequência.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p577.md`, com o hash do commit da medição.
- [ ] Só depois desta confirmação o estado de P576 passa de "a confirmar" para "fechado" de facto.
