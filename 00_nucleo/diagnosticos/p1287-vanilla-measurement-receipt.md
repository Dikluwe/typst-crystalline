# P1287 — recibo de medição vanilla

**Estado:** `BASELINE_MEASURED_WITH_UNKNOWNS`  
**Papel:** Autor exclusivo de baseline/medição.  
**Regime:** protocolo completo de materialização segregada, fase de baseline.  
**Janela:** `2026-08-30T22:15:14-03:00`–`2026-08-30T22:22:46-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.  
**Árvore:** working tree não commitida.  
**Atestação:** segregado por capacidades e artefatos, sem isolamento ambiental forte.

Este papel não leu código candidato em `01_core`–`04_wiring`, não executou
`target/debug/typst` nem `target/release/typst`, e não escreveu contrato, L0,
código, testes candidatos, ataques ou veredito. Escreveu somente este recibo.

## Entradas e identidade

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| Passo 1287 | `eb0622dd0fe195dfb4ed0dcdae079d0bc8c0162837531358a3598b7c9f0e32a8` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

`/usr/local/bin/typst --version` observou `typst 0.15.1 (e0e8ca4d)`.
O alvo declarado é upstream/main `a51e02804`; a associação dos bytes instalados
a esse commit permanece **Unknown como prova independente**. A identidade executável
deste baseline é o SHA-256 acima.

Ferramentas: Python `3.12.3`, Pillow `10.2.0`, qpdf `11.9.0`,
pdfinfo `26.05.0`, pdftotext/pdffonts `24.02.0`, mutool `1.23.10`,
ImageMagick `6.9.12-98 Q16`, libxml `2.15.2`, gzip `1.12`.
Hashes: qpdf `10fc302c4ca9860f24b8d2cb7f8a4cc454ba59d4a91e7a8e40f6b2c229486df7`;
pdfinfo `fee70ade670fb025343aca2b5c3a2aacacb8ed9edce1b716b233b5de924b6bf5`;
pdftotext `0fb98ea179e19154a90202608c164f2a319b79f16576fa6534b2d601033565e7`;
pdffonts `9855e69c629b8d67423864ba4657750efba3c76370ca02106b157668e9b50`;
mutool `9203440040cc38ee412aeedbfe57f452d6b85709c5b1600ca6ab2aa05478ab73`;
identify `9c47b9e24bfcf60e05ef327ea432742623ceb08d7a34e7356f38522447731e02`;
xmllint `4bbea5eef5797c061b683e7a702ed6222ac11bc6bd3fab63c8148f197bf47b5c`;
gzip `afea077ce127d4fa9ad410d3066ba2b54dea19c0b44f04adf56c72d5f7b7a9bb`.

## Política de `Unknown`

`Unknown` nunca é sucesso. Aplica-se a fenómeno sem fixture, construção opaca
não interpretável, dependência/formato ausente, baseline malformado, proveniência
ambígua, budget esgotado ou observação unilateral. `Available` significa somente
que o vanilla produziu o observável; nunca significa `Preserved`.

## Corpus amplo vanilla

Para cada `lab/parity/corpus/**/*.typ`, em ordem, foram executados PDF, PNG e SVG:

```sh
/usr/local/bin/typst compile --creation-timestamp 0 --jobs 1 \
  --diagnostic-format short --format FORMAT SOURCE OUTPUT
```

Resultado: 91 fontes, 273 invocações; por formato, 87 exits `0` e 4 exits `1`;
261 artefatos produzidos. Nenhum sucesso gerou mais de uma página. Isto é população
executada, não percentagem da linguagem.

- manifesto de todas as fontes/assets:
  `099ebaae4edc9a2fe0ce94b1e887074a37c932d29b59d04a0f839eb7a36bd291`;
- TSV temporário de 273 linhas:
  `a72558cdf8ee61142a12d5e2bf353d46dc45be91a0c3b066c5acc6b6b867d8ec`;
- manifesto dos 261 outputs:
  `1ba140892a3fe33da0c5f5e7014153bac9637c7b39a3bf5f5b1ecd5ea3297207`.

Rejeições, idênticas nos três formatos:

| Fonte | SHA-256 | Diagnóstico |
|---|---|---|
| `markup/error.typ` | `b96cd0c2f010df082bb1dbb302308e5a2ca3c7b312018edad730241e843a5858` | `unclosed delimiter` |
| `p500/test-calc-rest.typ` | `0c18b7dfd413c9cbaae74bf15e827a28768b493e88ea84fd8a3e2cc37b2c47a7` | `calc does not contain log10` |
| `p500/test-str-methods.typ` | `3562ece1150d132b68d76b3585236c8f081b384dd2906b54c0bf87a9f2f22b08` | `string has no method to-upper` |
| `semantic/funcao-builtin.typ` | `2e6783f014d16c3e706cb6856b6e29d74aa859b38514730a2ba01b786072e570` | `unknown variable len` |

O corpus cobre texto/Unicode, bidi, estilos, matemática, tabelas, colunas, place,
notas, contadores, referências, links, imagens, query e eval. `grid`, multipaginação
ampla e HTML standalone ficam **Unknown**. Eval: 10 expressões, 9 exits `0` e uma
rejeição; TSV SHA `f8937dd1a2b451bb0a2af1ea29f16d1206a3ce16953251ab2fc5c50c19b69c7e`.
Três queries reais (`metadata`, `heading`, `figure`) tiveram exit `0`;
stdout SHA, respectivamente, `6a08b1d4dafcfa7a1c30d449edd92e4b1b23641cf2b289f77f615e8d5316d318`,
`af30a0e506f6f94c0c488e762f1e3fb72cb9b2a793bdeb795dbcfd459555942d` e
`baa6aa182233d244fc56371c0e82fe5a0b51a959c94f926448920f9c965c4e99`.

## Focais PNG/PDF

Fixture plain SHA `9c168a60de075abeab24b85db0fe39259670378fafb3bc586fd7ae23efc335ae`.

PNG a 144 PPI, repetido duas vezes: bytes idênticos, SHA
`b6c64e93391ebac390ff2d85563801e434526b2629b9a0288a83731036cdfe22`;
`1191x1684`, sRGB, GrayscaleAlpha, 8-bit, 14264 bytes; RGBA decodificado SHA
`89170a876c7c245a9853302f064fbd9427db93d5fed2ca4e21d7c52f5e9541dd`;
769 pixels não brancos e 221 cores. Nenhuma tolerância foi autorizada.

PDF com timestamp 0, repetido: bytes idênticos, SHA
`bc678e4b68c0da5e6f774627aaaee81eb8d1ec2bf1073f51230476d686c7bacd`.
qpdf exit 0; 1 página A4, boxes `0 0 595.28 841.89 pt`; texto
`Hello, parity.` (SHA `fd2a1f3e7ef87eaf70aeade710e1b95de9d3db4904605e264b7b10e8c79fe4e6`);
fonte `DRQSPO+LibertinusSerif-Regular-Identity-H`, CID Type 0C, emb/subset/Unicode
`yes/yes/yes`; tagged, `/Marked true`, `/StructTreeRoot` presente. Nome da
fonte, texto, páginas/boxes e acessibilidade ficam eixos separados.

## Focais SVG/bundle

SVG plain repetido: SHA
`079b70a852ed455ba3f4da001a66904fec08219e08f79531ab7d8f6e695414c2`,
XML válido.

| Caso | compile/XML | SHA SVG | Baseline |
|---|---|---|---|
| tiling com conteúdo `[T]` | `0/4` | `452b7ba9e1e2921c3bd876e4068f99cf95c9b7ec35354c04ed2a1911a152f2e4` | **Unknown**: 1 NUL em `xlink:href` |
| tiling com PNG | `0/0` | `e1deb9b4f9d5de5d3fbfe7f55cd4a6ad3918bb90a1760135bd9c164b71dc6bf1` | Available |
| tiling com gradiente | `0/0` | `848f70b518f528cde97cf95d0716642f09d355826bff354c711bf8df7ce43023` | Available |
| duas fontes/glyph IDs | `0/0` | `133c3c879043e47ef34c4ec88e96425b5cb4b3dc001563d0c5f251c91de3d10b` | 2 IDs únicos; 0 hrefs não resolvidos |
| fonte ausente | `0/0` | `78f58a834bec5df09fc503fe61f3b188541bbb5e99a16c7b02018d50dfb90888` | aviso `unknown font family` |

O NUL do tiling por conteúdo também ocorre sem `--pretty`; SVG compacto SHA
`bdf63fa295707285ca6d4b0635bae80f8225eb2a64ab6888f28d4ca8ccc8007d`.
É baseline defect e nunca sucesso.

A fonte agregada de Linear/Luma, Radial/CMYK, Conic/CMYK, tilings, SVGZ,
clipping e even-odd teve compile 0, mas o NUL acima tornou o XML malformado.
Aceitação sintática foi medida; a árvore individual e o produto Conic por espaço
ficam **Unknown**. Um alfa isolado foi XML válido, SHA
`0980bcd1727ee4e51af850921a8a5d5fb71ef061aac8112f451b4e6ddac64312`;
o vanilla usou cores hex RGBA, 2 linearGradient e 0 mask. Máscara específica
não recebe crédito. SVGZ SHA `686871470ae0cceab12e84c0fce65ab533ec73c4ebab7f8dad8d5ce3079b14ce`
foi aceito e embutido como data URI gzip/base64; conteúdo interno comparado fica
**Unknown**. JPEG/GIF/WebP/PDF-as-image, deps SVG externas e wrapper sem fonte
ficam **Unknown**.

Bundle `--features bundle,html` teve exit 0 e produziu:
`index.html` SHA `9da7d8d4a376219fe3a22e74d54efbc9949a270b775578679479120c8b2d3ba6`
com `nested/b.svg#b`; `nested/b.svg` SHA
`f4550f80099adcf8425071f6a8cc1666b50a806081466262f33203c1a0deec72`
com `../index.html`; e `c.pdf` SHA
`97150eb1276244b07029014be1ffe57bde13f984b045da63691f89c923adf728`,
2 páginas, link cross-file e destino interno. Manifesto:
`0e3001cdbae10108977a95c3c3da700b48c4a6c6028e9bcd117a8fef0fd59986`.
Isto prova disponibilidade vanilla, não equivalência bilateral/viewer.

## Proveniência da árvore

Snapshot pré-receipt em `2026-08-30T22:22:46-03:00`:

```text
git diff HEAD --stat: 124 files changed, 647273 insertions(+), 1789 deletions(-)
SHA-256 da saída: 8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c
SHA-256 de git status --short: aaccc8c5ff6b879c869ae3fe4fefa7b40c902721931563322f8362b2b9c34836
```

A árvore é compartilhada e já estava suja; os números só podem ser transportados
com os hashes de input, comandos e binário acima. Nenhum veredito global é emitido.

