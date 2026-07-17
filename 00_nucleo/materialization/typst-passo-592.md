---
# P592 — A diferença residual de 12,71 pontos é um espaço a mais?

> **Passo:** 592
> **Data:** 2026-07-05
> **Foco:** P591 mediu uma diferença de posição de ~12,71 pontos entre cristalino e vanilla, e classificou-a como "aceitável, não afecta a quebra", sem investigar. 12,71 pontos é quase exactamente a largura de um espaço em DejaVu Sans a 40 pontos — o mesmo tipo de coincidência numérica que, em P587, levou à descoberta de um espaço a mais no início do parágrafo. Este passo confirma se é o mesmo tipo de causa, ou outra coisa, antes de aceitar a diferença como está.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Uma diferença que bate certo com um número conhecido (largura de espaço) não se aceita sem se perguntar porquê — foi assim que P587 encontrou a causa real da última vez.
> **Dependências:** P591 (onde a diferença apareceu e foi aceite sem investigação), P587 (precedente directo do mesmo tipo de pista).

---

## Verificação

### Confirmar a largura exacta de um espaço em DejaVu Sans a 40pt

```bash
python3 -c "
from fontTools.ttLib import TTFont
f = TTFont('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')
upem = f['head'].unitsPerEm
space_gid = f.getBestCmap()[ord(' ')]
space_width = f['hmtx'][space_gid][0]
print(f'upem={upem}, space_width_units={space_width}, at 40pt={space_width/upem*40:.4f}pt')
"
```

Comparar o valor exacto com os 12,71 pontos medidos em P591. Se baterem certo (dentro de décimas), confirma a suspeita.

### Confirmar onde entra o espaço a mais (ou a menos)

```bash
grep -n "space_width\|Content::Space" 03_infra/src/layout_bidi.rs | head -20
```

Perguntas, com `file:line`:

1. Depois da correcção de P587 (não avançar cursor para espaço no início de linha), existe algum ponto dentro de `layout_bidi.rs` (a passagem de reordenação RTL, criada depois de P587) que ainda conte um espaço inicial ou final de forma diferente entre a lógica de reordenação e a lógica normal de layout?
2. O deslocamento é sempre de exactamente um `space_width`, ou varia consoante o documento? Testar com um documento RTL diferente (por exemplo, só três palavras, sem número no meio) para confirmar se a diferença de 12,71 pontos aparece sempre, ou só neste caso específico.

```bash
cat > /tmp/p592-tres-palavras.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب على الطاولة
EOF
./target/release/typst /tmp/p592-tres-palavras.typ /tmp/p592.pdf
pdftotext -tsv /tmp/p592.pdf -
lab/typst-original/target/release/typst compile /tmp/p592-tres-palavras.typ /tmp/p592-vanilla.pdf
pdftotext -tsv /tmp/p592-vanilla.pdf -
```

### Critério de fecho

- [ ] Largura exacta de espaço confirmada e comparada com os 12,71 pontos.
- [ ] Confirmado se a diferença aparece sempre (mesmo sem o número "42" no meio) ou só neste caso.
- [ ] Localizado, com `file:line`, se há um espaço a mais ou a menos na passagem de reordenação RTL.

---

## Decisão

Se a diferença for confirmada como um espaço mal contado: corrigir, seguindo o mesmo padrão de P587 — não deixar como "resíduo aceitável".

Se a diferença não bater certo com a largura de espaço, ou não aparecer de forma consistente noutros documentos: registar isso, com os números, e então sim aceitar como resíduo pequeno, mas com prova, não com suposição.

---

## Critério de fecho do passo

- [ ] Largura de espaço comparada com os 12,71 pontos.
- [ ] Consistência testada noutro documento.
- [ ] Causa localizada, ou explicitamente descartada com prova.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p592.md`, com hash do commit.
- [ ] Tabela de estado da sequência RTL só marca "fechado" de vez depois deste passo confirmar ou corrigir isto.
