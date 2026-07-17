---
# P669 — Confirmar o ramo vazio do dispatch de fontes

> **Passo:** 669
> **Data:** 2026-07-10
> **Foco:** P668 corrigiu o ramo "uma fonte" do dispatch em `pipeline.rs`, redireccionando para o caminho multi-font quando é uma VF com eixos não-default. O `match` tem um terceiro ramo, "nenhuma fonte resolvida" (`[]`), nunca testado neste contexto. Dado que esta pendência (P525) já surpreendeu três vezes seguidas com caminhos não testados, esta última verificação, pequena, fecha o assunto com confiança maior do que assumir que o caso vazio não tem nada a ver com isto.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P668 (onde o ramo "uma fonte" foi corrigido).

---

## Verificação

### Confirmar quando o ramo vazio é atingido

```bash
grep -n "resolved.as_slice()\|\[\] =>" 03_infra/src/pipeline.rs
```

Confirmar em que situação `resolved` fica vazio — provavelmente um documento sem texto nenhum, ou só com elementos que não precisam de fonte (formas, imagens).

### Testar directamente com fonte variável declarada mas não usada

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
cat > /tmp/p669-vazio.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700)
#rect(width: 10pt, height: 10pt)
EOF
./target/release/typst /tmp/p669-vazio.typ /tmp/p669.pdf
echo "Exit code: $?"
```

Confirmar se um documento que declara `weight: 700` mas nunca chega a desenhar texto (só uma forma) passa pelo ramo vazio sem problema, ou se `resolved` inclui a fonte mesmo sem texto visível.

### Critério de fecho

- [ ] Confirmado em que situação o ramo vazio é atingido.
- [ ] Confirmado que não há forma de uma fonte variável com eixos não-default acabar neste ramo sem ser instanciada.

---

## Decisão

Se o ramo vazio nunca envolver fontes variáveis (por definição, não há texto, logo não há necessidade de instanciar nada): confirmado, sem código a mudar.

Se houver algum caso onde uma fonte é resolvida mas cai no ramo vazio por outra razão: tratar como P668 tratou o ramo de uma fonte.

---

## Critério de fecho do passo

- [ ] Ramo vazio confirmado como irrelevante para fontes variáveis, ou corrigido se não for.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p669.md`.
- [ ] Pendência de P525 — declarada fechada de vez, com os três ramos do dispatch confirmados, não só dois.
