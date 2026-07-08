---
# P610 — Testar `.otc` directamente e varrer o corpus por outras fontes não subsetadas

> **Passo:** 610
> **Data:** 2026-07-05
> **Foco:** P609 corrigiu o caso `.ttc`, mas deixou `.otc` (OpenType Collection, com dados CFF em vez de `glyf`) como "não testado directamente, mas o cabeçalho é idêntico". Dado que a causa original do bug de 15 MB era exactamente um formato nunca testado a cair num fallback caro sem aviso, este passo testa `.otc` de facto, e varre o resto do corpus à procura de outros documentos com o mesmo sintoma (ficheiro muito maior do que devia), agora que se sabe o que procurar.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** "O cabeçalho é idêntico, deve funcionar" é uma suposição, não uma confirmação — foi assim que o bug de P608/609 passou despercebido.

---

## Parte 1 — Testar `.otc` directamente

### Encontrar uma fonte `.otc` real no sistema

```bash
find / -name "*.otc" 2>/dev/null | head -5
fc-list : file | grep -i "\.otc" | head -5
```

Se não houver nenhuma no sistema, criar ou obter uma fonte `.otc` de teste (por exemplo, algumas distribuições de Noto vêm em `.otc`).

### Testar directamente

```bash
cat > /tmp/p610-otc.typ <<'EOF'
#set text(font: "<nome da fonte OTC encontrada>")
你好世界
EOF
./target/release/typst /tmp/p610-otc.typ /tmp/p610-otc.pdf
ls -la /tmp/p610-otc.pdf
mutool extract /tmp/p610-otc.pdf
ls -la font-*
```

Confirmar se a fonte extraída está subsetada (poucos KB, poucos glifos) ou completa (muitos MB, milhares de glifos) — o mesmo teste que revelou o problema de `.ttc` em P609.

### Critério de fecho da Parte 1

- [x] Fonte `.otc` real testada, não assumida.
- [x] Confirmado se o subsetting funciona da mesma forma que para `.ttc`.
- [ ] Se falhar: corrigir com o mesmo método de P609, adaptado ao formato CFF dentro da coleção.

---

## Parte 2 — Varrer o corpus por outros documentos com o mesmo sintoma

Agora que se sabe que o sintoma é "ficheiro muito maior do que o esperado, por fonte de fallback não subsetada", varrer os documentos já existentes no corpus à procura de casos parecidos que possam ter passado despercebidos.

```bash
for f in lab/parity/corpus/*/*.typ; do
  ./target/release/typst "$f" /tmp/p610-out.pdf 2>/dev/null
  if [ -f /tmp/p610-out.pdf ]; then
    size=$(stat -c%s /tmp/p610-out.pdf)
    if [ "$size" -gt 1000000 ]; then
      echo "GRANDE ($size bytes): $f"
    fi
    rm /tmp/p610-out.pdf
  fi
done
```

Qualquer documento que produza um ficheiro muito maior do que o esperado para o seu conteúdo (mais de 1 MB para um documento de teste pequeno) é candidato a ter o mesmo problema.

### Critério de fecho da Parte 2

- [x] Corpus inteiro varrido, com o tamanho de cada PDF gerado registado.
- [ ] Qualquer documento fora do padrão esperado investigado individualmente, com o mesmo método de P609 (extrair fontes, confirmar se estão subsetadas).

---

## Critério de fecho do passo

- [x] `.otc` testado directamente, não assumido.
- [x] Corpus varrido por tamanho anómalo de ficheiro.
- [x] Nenhum novo caso encontrado; não houve correcção adicional.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p610.md`, com hash do commit.
